

use super::inner::{CameraRayGenerator, RendererHelper};
use {Color, RenderSettings};
use color;
use math::{ApproxEq, Dot, Norm, Ray3f, Real};
use rand;
use rand::Closed01;
use traits::{RenderCamera, Renderer, SceneHandler, Surface};
use utils::consts;

pub struct PathTracer {
    ray_gen: CameraRayGenerator,
    setup: RenderSettings,

    /// (brdf, light sources)
    di_samples_weight: Option<(Real, Real)>,
}

impl PathTracer {
    pub fn new(setup: &RenderSettings) -> PathTracer {
        PathTracer {
            ray_gen: CameraRayGenerator::new(),
            setup: *setup,
            di_samples_weight: None,
        }
    }

    pub fn with_direct_illumination(
        mut self,
        brdf_weight: Real,
        light_sources_weight: Real,
    ) -> Self {
        let mut brdf_w = brdf_weight;
        let mut ls_w = light_sources_weight;
        if brdf_w + ls_w > 1.0 {
            let sum = brdf_w + ls_w;
            brdf_w /= sum;
            ls_w /= sum;
        }
        self.di_samples_weight = Some((brdf_w, ls_w));
        self
    }

    fn trace_path_rec<S>(&self, scene: &S, ray: &Ray3f, depth: u32) -> Color
    where
        S: SceneHandler + ?Sized,
    {

        if depth == self.setup.path_depth {
            return color::BLACK;
        }

        let di_enable = self.di_samples_weight.is_some();

        match scene.intersection(ray) {
            Some(ref sp) if sp.normal.dot(&(-ray.dir)) > 0.0 => {
                let mat = sp.bsdf.as_ref();

                let le = if let Some(c) = mat.radiance() {
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

                        if let Some((lp, pdf_ls)) = scene
                            .light_sources()
                            .sample((&sp.position, &sp.normal), Surface::sample_surface_d_proj)
                        {
                            let shadow_ray =
                                Ray3f::new(&sp.position, &(lp.position - sp.position).normalize());
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);
                            let cos_theta_l = lp.normal.dot(&(-shadow_ray.dir));

                            if cos_theta > 0.0 && cos_theta_l > 0.0 {
                                if let Some(ip) = scene.intersection(&shadow_ray) {
                                    if ip.position.approx_eq_eps(
                                        &lp.position,
                                        &(consts::POSITION_EPSILON * 2.0),
                                    ) {

                                        let (fr, pdf_brdf) = sp.bsdf
                                            .eval_proj(&sp.normal, &ray.dir, &shadow_ray.dir);
                                        let pdf_sum_inv = 1.0 / (pdf_brdf * brdf_w + pdf_ls * ls_w);
                                        let le = lp.bsdf.radiance().unwrap();

                                        di = (fr * le) * (pdf_sum_inv as f32);
                                    }
                                }
                            }
                        }
                    } else {
                        // brdf sampling
                        let (brdf_ray_dir, _, _) = sp.bsdf.sample_proj(&sp.normal, &ray.dir);
                        let shadow_ray = Ray3f::new(&sp.position, &brdf_ray_dir);

                        if let Some(ip) = scene.intersection(&shadow_ray) {
                            if let Some(le) = ip.bsdf.radiance() {

                                let pdf_ls = scene.light_sources().pdf(
                                    ip.surface,
                                    (&ip.position, &ip.normal),
                                    (&sp.position, &sp.normal),
                                    Surface::pdf_d_proj,
                                );
                                let (fr, pdf_brdf) =
                                    sp.bsdf.eval_proj(&sp.normal, &ray.dir, &shadow_ray.dir);

                                let pdf_sum_inv = 1.0 / (pdf_brdf * brdf_w + pdf_ls * ls_w);
                                let res = (fr * le) * (pdf_sum_inv as f32);
                                di += res;
                            }
                        }
                    }

                    di

                } else {
                    color::BLACK
                };

                let (new_ray_dir, fr, pdf_p) = sp.bsdf.sample_proj(&sp.normal, &ray.dir);
                let new_ray = Ray3f::new(&sp.position, &new_ray_dir);
                let li = self.trace_path_rec::<S>(scene, &new_ray, depth + 1);
                let indirect_illumination = (fr * li) * (1.0 / pdf_p) as f32;

                le + (direct_illumination + indirect_illumination)
                // le + direct_illumination;
            }
            _ => color::BLACK,
        }
    }
}

impl<S: SceneHandler + ?Sized, C: RenderCamera + ?Sized> RendererHelper<S, C> for PathTracer {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, _: &RenderSettings) -> Color {
        self.trace_path_rec::<S>(scene, initial_ray, 0)
    }

    fn get_ray(&self, _: &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S: SceneHandler + ?Sized, C: RenderCamera + ?Sized> Renderer<S, C> for PathTracer {
    fn pre_render(&mut self, _: &S, camera: &C, _: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}
