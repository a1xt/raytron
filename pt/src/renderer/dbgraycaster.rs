use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm};
use color;
use traits::{Renderer, SceneHandler, RenderCamera};

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

    fn trace_path_rec<S>(&self, scene: &S, ray: &Ray3f, _: u32) -> Color
        where S: SceneHandler + ?Sized
    {

        if let Some(sp) = scene.intersection(ray) {
            let mat = sp.bsdf;
            if let Some(c) = mat.radiance() {
                return c;
            } else {
                if let Some(light) = scene.light_sources().iter().into_iter().next() {
                    let (light_point, _) = light.sample_surface_p((&sp.position, &sp.normal));
                    let shadow_ray = Ray3f::new(&sp.position, &(light_point.position - sp.position).normalize());                    

                    let color = Color::new((0.5 + sp.normal.x * 0.5) as f32,
                                           (0.5 + sp.normal.y * 0.5) as f32,
                                           (0.5 + sp.normal.z * 0.5) as f32);
                    //return color;
                    if let Some(lp) = scene.intersection(&shadow_ray){
                        
                        if lp.bsdf.radiance().is_some() {
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);

                            return color * (cos_theta as f32);

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

impl<S: SceneHandler + ?Sized, C: RenderCamera + ?Sized> RendererHelper<S, C> for DbgRayCaster {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, _: &RenderSettings) -> Color {
        self.trace_path_rec::<S>(scene, initial_ray, 0)
    }

    fn get_ray(&self, _ : &C, x: u32, y: u32) -> Ray3f {
        self.ray_gen.get_ray(x, y)
    }
}

impl<S: SceneHandler + ?Sized, C: RenderCamera + ?Sized> Renderer<S, C> for DbgRayCaster {
    fn pre_render(&mut self, _: &S, camera: &C, _: &RenderSettings) {
        self.ray_gen = CameraRayGenerator::with_camera(camera);
    }
}