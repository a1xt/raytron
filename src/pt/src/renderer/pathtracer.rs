use ::{RenderSettings, Color};
use math::{self, Ray3f, Dot, Norm, Coord, ApproxEq};
use color;
use traits::{Renderer, SceneHolder, RenderCamera, Surface};
use std;
use std::f32::consts::PI;
use rand::{Closed01};
use rand;
use utils;
use utils::consts;

use super::inner::{RendererHelper, CameraRayGenerator};

pub struct PathTracer {
    ray_gen: CameraRayGenerator,
    setup: RenderSettings,

    /// (brdf, light sources)
    //di_samples_num: Option<(u32, u32)>,
    di_enable: bool,
}

impl PathTracer {
    pub fn new (setup: &RenderSettings) -> PathTracer {
        PathTracer {
            ray_gen: CameraRayGenerator::new(),
            setup: *setup,
            //di_samples_num: None,
            di_enable: false,
        }
    }

    //pub fn with_direct_illumination (mut self, brdf_samples: u32, light_sources_samples: u32) -> Self {
    pub fn with_direct_illumination(mut self) -> Self {
        // if brdf_samples >= 1 && light_sources_samples >= 1 {
        //     self.di_samples_num = Some((brdf_samples, light_sources_samples));
        // }

        self.di_enable = true;
        self
    }

    fn trace_path_rec<S, C>(&self, scene: &S, ray: &Ray3f, depth: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if depth == self.setup.path_depth {
                return color::BLACK;
        }

        // let (di_enable, bsdf_n, ls_n) = {
        //     if let Some((bsdf, ls)) = self.di_samples_num {
        //         (true, bsdf, ls)
        //     } else {
        //         (false, 0, 0)
        //     }
        // };

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.material;

            let Le = if let Some(c) = mat.emission() {
                if depth > 0 && self.di_enable {
                    color::BLACK
                } else {
                    c
                }
            } else {
                color::BLACK
            };

            let direct_illumination = if self.di_enable {
                let mut di = color::BLACK;

                // light source sampling
                if let Some((lp, pdf_ls)) = utils::sample_surfaces(scene.light_sources(), &sp.position) {
                    let shadow_ray = Ray3f::new(&sp.position, &(lp.position - sp.position).normalize());
                    let cos_theta = sp.normal.dot(&shadow_ray.dir);
                    let cos_theta_l = lp.normal.dot(&(-shadow_ray.dir));

                    if cos_theta > 0.0 && cos_theta_l > 0.0 {
                        if let Some(ip) = scene.intersection_with_scene(&shadow_ray) {
                            if ip.position.approx_eq_eps(&lp.position, &(consts::POSITION_EPSILON * 2.0)) {

                                let r2 = (sp.position - lp.position).norm_squared();
                                let g = cos_theta * cos_theta_l / r2;
                                let pdf_brdf = sp.material.pdf_proj(&sp.normal, &ray.dir, &shadow_ray.dir) * g;
                                let pdf_sum_inv = 1.0 / (pdf_brdf + pdf_ls);

                                let fr = sp.material.reflectance(&sp.normal, &ray.dir, &shadow_ray.dir);
                                let le = lp.material.emission().unwrap();
                                
                                di = color::mul_s(&color::mul_v(&fr, &le), (g * pdf_sum_inv) as f32);
                            }
                        }
                    }
                }

                // brdf sampling
                {
                    let (brdf_ray_dir, _, _) = sp.material.sample_bsdf_proj(&sp.normal, &ray.dir);
                    let shadow_ray = Ray3f::new(&sp.position, &brdf_ray_dir);

                    if let Some(ip) = scene.intersection_with_scene(&shadow_ray) {
                        if let Some(le) = ip.material.emission() {

                            let r2 = (sp.position - ip.position).norm_squared();
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);
                            let cos_theta_l = ip.normal.dot(&(-shadow_ray.dir));
                            let g_inv = r2 / (cos_theta * cos_theta_l);

                            let pdf_ls = ip.surface.pdf(&ip.position, &sp.position) * g_inv;
                            let pdf_brdf = sp.material.pdf_proj(&sp.normal, &ray.dir, &shadow_ray.dir);
                            let pdf_sum_inv = 1.0 / (pdf_brdf + pdf_ls);

                            let fr = sp.material.reflectance(&sp.normal, &ray.dir, &shadow_ray.dir);

                            let res = color::mul_s(&color::mul_v(&fr, &le), pdf_sum_inv as f32);
                            di = color::sum(&di, &res);
                        }
                    }
                } 

                di

            } else {
                color::BLACK
            };

            let (new_ray_dir, fr, pdf_p) = sp.material.sample_bsdf_proj(&sp.normal, &ray.dir);
            let new_ray = Ray3f::new(&sp.position, &new_ray_dir);
            let Li = self.trace_path_rec::<S, C>(scene, &new_ray, depth + 1);
            let indirect_illumination = color::mul_s(&color::mul_v(&fr, &Li), (1.0 / pdf_p) as f32);

            return color::sum(&Le, &color::sum(&direct_illumination, &indirect_illumination));

        //     let light_samples = 1.0;
        //     let brdf_samples = 1.0;

        //     let mut direct_lighting = color::BLACK;

        //     //for _ in 0..di_samples_num {
        //     if light_samples > 0.0 {
            
        //         if let Some(light) = scene.random_light_source() {
        //             let (lsource_point, pdf_l) = light.sample_surface(&shifted_ray_pos);
                  
        //             let shadow_ray = Ray3f::new(shifted_ray_pos, (lsource_point.position - shifted_ray_pos).normalize());
        //             let cos_theta = sp.normal.dot(&shadow_ray.dir);
        //             let cos_theta_l = light.normal_at(&lsource_point.position).dot(&shadow_ray.dir) * (-1.0);
        //             if cos_theta > 0.0 && cos_theta_l > 0.0 {                    
        //                 let di = if let Some(lp) = scene.intersection_with_scene(&shadow_ray) {
        //                     let mut res = color::BLACK;
        //                     if let Some(e) = lp.material.emission() {
        //                         let r2 = (shifted_ray_pos - lp.position).norm_squared();
                                
        //                         let pdf_brdf = (1.0 / PI as Coord) * (cos_theta * cos_theta_l) / r2;
        //                         let pdf_sum_inv = 1.0 / (light_samples * pdf_l + brdf_samples * pdf_brdf);

        //                         let d = (cos_theta * cos_theta_l) / r2;
        //                         let fr = color::mul_s(&sp.material.reflectance(&ray.dir, &shadow_ray.dir, &sp.normal), 1.0 / PI);

        //                         res = color::mul_v(&e, &color::mul_s(&fr, (d * pdf_sum_inv ) as f32));                       
        //                     }

        //                     res
        //                 } else {
        //                     color::BLACK
        //                 };

        //                 direct_lighting = color::sum(&direct_lighting, &di);
        //             }
        //         }
        //     }

        //     if brdf_samples > 0.0 {
            
        //         let brdf_ray_dir = math::hs_cosine_sampling(&sp.normal);
        //         let brdf_shadow_ray = Ray3f::new(shifted_ray_pos, brdf_ray_dir);
                
        //         if let Some(lp) = scene.intersection_with_scene(&brdf_shadow_ray) {
        //             if let Some(e) = lp.material.emission(){
        //                 let cos_theta = sp.normal.dot(&brdf_ray_dir);
        //                 let cos_theta_l = lp.normal.dot(&brdf_ray_dir) * (-1.0);
        //                 let r2 = (shifted_ray_pos - lp.position).norm_squared();

        //                 let fr = color::mul_s(&sp.material.reflectance(&ray.dir, &brdf_ray_dir, &sp.normal), 1.0 as f32 / PI);

        //                 let light = scene.random_light_source().unwrap();
        //                 let pdf_l = (1.0 / light.area()) * (r2 / (cos_theta * cos_theta_l));
        //                 let pdf_brdf = (1.0 / PI as Coord);// * (cos_theta * cos_theta_l) / r2;
        //                 let pdf_sum_inv = 1.0 / (light_samples * pdf_l + brdf_samples * pdf_brdf);

        //                 let res = color::mul_v(&e, &fr);

        //                 // if rand::random::<u16>() % 10000 == 0 {
        //                 //     println!("pdf_l = {}", pdf_l);
        //                 // }

        //                 direct_lighting = color::sum(&direct_lighting, &color::mul_s(&res, pdf_sum_inv as f32));
    
        //             } 
        //         }
        //     }

            

       
            
        //     let (mut new_ray, reflectance) = mat.brdf(&ray.dir, &shifted_ray_pos, &sp.normal);
        //     let li: Color = self.trace_path_rec::<S, C>(scene, &new_ray, depth + 1);

        //     let indirect_lighting = &color::mul_v(&reflectance, &li);
        //     return color::sum(&direct_lighting, &indirect_lighting);
        //    // return direct_lighting;


        } else {
            return color::BLACK;
        }

    
    }


}

impl<S: SceneHolder, C: RenderCamera> RendererHelper<S, C> for PathTracer {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color {
        let mut res = self.trace_path_rec::<S, C>(scene, &initial_ray, 0);

        res        
    }

    
    
    fn get_ray(&self, _ : &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S:SceneHolder + Sync, C: RenderCamera + Sync> Renderer<S, C> for PathTracer {
    fn pre_render(&mut self, scene: &S, camera: &C, setup: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}