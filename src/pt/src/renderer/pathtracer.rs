use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm,ApproxEq, Real};
use color;
use traits::{Renderer, SceneHolder, RenderCamera, Surface};
use utils;
use utils::consts;
use rand;
use rand::{Closed01};

use super::inner::{RendererHelper, CameraRayGenerator};

pub struct PathTracer {
    ray_gen: CameraRayGenerator,
    setup: RenderSettings,

    /// (brdf, light sources)
    di_samples_weight: Option<(Real, Real)>,
}

impl PathTracer {
    pub fn new (setup: &RenderSettings) -> PathTracer {
        PathTracer {
            ray_gen: CameraRayGenerator::new(),
            setup: *setup,
            di_samples_weight: None,
        }
    }

    pub fn with_direct_illumination(mut self, brdf_weight: Real, light_sources_weight: Real) -> Self {
        let mut brdf_w = brdf_weight;
        let mut ls_w = light_sources_weight;
        if brdf_w + ls_w > 1.0 {
            let sum = brdf_w + ls_w;
            brdf_w = brdf_w / sum;
            ls_w = ls_w / sum;
        }
        self.di_samples_weight = Some((brdf_w, ls_w));
        self
    }

    fn trace_path_rec<S, C>(&self, scene: &S, ray: &Ray3f, depth: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if depth == self.setup.path_depth {
                return color::BLACK;
        }

        let di_enable = if let Some(_) = self.di_samples_weight {
            true
        } else {
            false
        };

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.bsdf.as_ref();

            let le = if let Some(c) = mat.emittance() {
                if depth > 0 && di_enable {
                    color::BLACK
                } else {
                    c
                }
            } else {
                color::BLACK
            };

            let direct_illumination = if let Some((brdf_w, ls_w)) = self.di_samples_weight {
                let mut di = color::BLACK;

                let Closed01(e) = rand::random::<Closed01<Real>>();
                if e > brdf_w {
                    // light source sampling
                    if let Some((lp, pdf_ls)) = utils::sample_surfaces::by_area(scene.light_sources(),
                                                                                (&sp.position, &sp.normal),
                                                                                Surface::sample_surface_d_proj) {
                        let shadow_ray = Ray3f::new(&sp.position, &(lp.position - sp.position).normalize());
                        let cos_theta = sp.normal.dot(&shadow_ray.dir);
                        let cos_theta_l = lp.normal.dot(&(-shadow_ray.dir));

                        if cos_theta > 0.0 && cos_theta_l > 0.0 {
                            if let Some(ip) = scene.intersection_with_scene(&shadow_ray) {
                                if ip.position.approx_eq_eps(&lp.position, &(consts::POSITION_EPSILON * 2.0)) {

                                    let (fr, pdf_brdf) = sp.bsdf.eval_proj(&sp.normal, &ray.dir, &shadow_ray.dir);
                                    let pdf_sum_inv = 1.0 / (pdf_brdf * brdf_w + pdf_ls * ls_w);
                                    let le = lp.bsdf.emittance().unwrap();
                                    
                                    di = color::mul_s(&color::mul_v(&fr, &le), pdf_sum_inv as f32);
                                }
                            }
                        }
                    }
                } else {

                    // brdf sampling
                    let (brdf_ray_dir, _, _) = sp.bsdf.sample_proj(&sp.normal, &ray.dir);
                    let shadow_ray = Ray3f::new(&sp.position, &brdf_ray_dir);

                    if let Some(ip) = scene.intersection_with_scene(&shadow_ray) {
                        if let Some(le) = ip.bsdf.emittance() {

                            let pdf_ls = utils::sample_surfaces::by_area_pdf(ip.surface,
                                                                             scene.light_sources(), 
                                                                             (&ip.position, &ip.normal), 
                                                                             (&sp.position, &sp.normal),
                                                                             Surface::pdf_d_proj);
                            let (fr, pdf_brdf) = sp.bsdf.eval_proj(&sp.normal, &ray.dir, &shadow_ray.dir);
                            let pdf_sum_inv = 1.0 / (pdf_brdf * brdf_w + pdf_ls * ls_w);

                            let res = color::mul_s(&color::mul_v(&fr, &le), pdf_sum_inv as f32);
                            di = color::sum(&di, &res);
                        }
                    }
                } 

                di

            } else {
                color::BLACK
            };

            let (new_ray_dir, fr, pdf_p) = sp.bsdf.sample_proj(&sp.normal, &ray.dir);
            let new_ray = Ray3f::new(&sp.position, &new_ray_dir);
            let li = self.trace_path_rec::<S, C>(scene, &new_ray, depth + 1);
            let indirect_illumination = color::mul_s(&color::mul_v(&fr, &li), (1.0 / pdf_p) as f32);

            return color::sum(&le, &color::sum(&direct_illumination, &indirect_illumination));
            // return color::sum(&le, &direct_illumination);

        } else {
            return color::BLACK;
        }
    }
    
}

impl<S: SceneHolder, C: RenderCamera> RendererHelper<S, C> for PathTracer {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, _: &RenderSettings) -> Color {
        let res = self.trace_path_rec::<S, C>(scene, &initial_ray, 0);
        res        
    }
    
    fn get_ray(&self, _ : &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S:SceneHolder + Sync, C: RenderCamera + Sync> Renderer<S, C> for PathTracer {
    fn pre_render(&mut self, _: &S, camera: &C, _: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}