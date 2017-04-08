use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm};
use color;
use traits::{Renderer, SceneHolder, RenderCamera};

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

    fn trace_path_rec<S, C>(&self, scene: &S, ray: &Ray3f, _: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.bsdf;
            if let Some(c) = mat.emittance() {
                return c;
            } else {
                if let Some(light) = scene.light_sources().into_iter().next() {
                    let (light_point, _) = light.sample_surface_p((&sp.position, &sp.normal));
                    let shadow_ray = Ray3f::new(&sp.position, &(light_point.position - sp.position).normalize());                    

                    let color = Color{data: [sp.normal.x.abs() as f32, sp.normal.y.abs() as f32, sp.normal.z.abs() as f32, 1.0]};

                    if let Some(lp) = scene.intersection_with_scene(&shadow_ray){
                        
                        if let Some(_) = lp.bsdf.emittance() {
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);

                            return color::mul_s(&color, cos_theta as f32);
                        }                      
                    } 
                }
            }
        } else {
            return color::BLACK;
        }
        color::BLACK
    
    }

}

impl<S: SceneHolder, C: RenderCamera> RendererHelper<S, C> for DbgRayCaster {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, _: &RenderSettings) -> Color {
        let res = self.trace_path_rec::<S, C>(scene, &initial_ray, 0);

        res        
    }

    fn get_ray(&self, _ : &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S: SceneHolder + Sync, C: RenderCamera + Sync> Renderer<S, C> for DbgRayCaster {
    fn pre_render(&mut self, _: &S, camera: &C, _: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}