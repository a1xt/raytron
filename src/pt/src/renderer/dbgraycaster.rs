use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm};
use color;
use traits::{Renderer, SceneHolder, RenderCamera};
use std::f32::consts::PI;
use rand::{Closed01};
use rand;
use utils::consts;

use super::inner::{RendererHelper, CameraRayGenerator};


pub struct DbgRayCaster {
    ray_gen: CameraRayGenerator,
}

impl DbgRayCaster {
    pub fn new () -> DbgRayCaster {
        DbgRayCaster {
            ray_gen: CameraRayGenerator::new(),
        }
    }

    fn trace_path_rec<S, C>(&self, scene: &S, ray: &Ray3f, depth: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.material;
            if let Some(c) = mat.emission() {
                return c;
            } else {
                if let Some(light) = scene.light_sources().next() {
                    let (light_point, _) = light.sample_surface(&sp.position);

                    let mut shadow_ray = Ray3f::new(&sp.position, &(light_point.position - sp.position).normalize());
                    //shadow_ray.origin += sp.normal * consts::RAY_SHIFT_DISTANCE;
                    

                    let color = Color{data: [sp.normal.x.abs() as f32, sp.normal.y.abs() as f32, sp.normal.z.abs() as f32, 1.0]}; 


                    if let Some(lp) = scene.intersection_with_scene(&shadow_ray){
                        
                        if let Some(_) = lp.material.emission() {
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);

                            return color::mul_s(&color, cos_theta as f32);
                        }
                         else {
                            if lp.normal.dot(&shadow_ray.dir) > 0.0 {
                                let t = (lp.position - shadow_ray.origin).norm();
                                if rand::random::<u32>() % 10000 <= 2 {
                                    //println!("tt = {}", t);
                                }
                                

                                //return color::RED;
                            }
                            //return color::RED;
                        }
                        
                    } 
                    // else {
                    //     return color::BLUE;
                    // }
                } 
                // else {
                //     return Color{data:[0.0, 1.0, 1.0, 1.0]};
                // }
            }
        } else {
            //Color {data: [1.0, 0.5, 0.5, 1.0f32]}
            return color::BLACK;
            //return color::GREEN;
        }

        color::BLACK
    
    }

}

impl<S: SceneHolder, C: RenderCamera> RendererHelper<S, C> for DbgRayCaster {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color {
        let mut res = self.trace_path_rec::<S, C>(scene, &initial_ray, 0);

        res        
    }

    fn get_ray(&self, _ : &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S: SceneHolder + Sync, C: RenderCamera + Sync> Renderer<S, C> for DbgRayCaster {
    fn pre_render(&mut self, scene: &S, camera: &C, setup: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}