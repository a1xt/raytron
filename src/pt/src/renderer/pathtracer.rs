use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm};
use color;
use traits::{Renderer, SceneHolder, RenderCamera};
use std;
use std::f32::consts::PI;
use rand::{Closed01};
use rand;
use utils::consts;

use super::inner::{RendererHelper, CameraRayGenerator};

pub struct PathTracer {
    ray_gen: CameraRayGenerator,
}

impl PathTracer {
    pub fn new () -> PathTracer {
        PathTracer {
            ray_gen: CameraRayGenerator::new(),
        }
    }

    fn trace_path_rec<S, C>(&self, scene: &S, ray: &Ray3f, depth: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.material;
            let le = if let Some(c) = mat.emission() {
                c
            } else { // light not found
                color::BLACK
            };

            if depth == 0 {
                return color::BLACK;
            }

            // let new_ray = mat.reflect_ray(&ray.dir, &sp.position, &sp.normal);
            
            // let cos_theta = new_ray.dir.dot(&sp.normal);

            // //let k = 2.0 * cos_theta;
            // let k = 1.0;        

            // //println!("reflectance: {:?}, 1/pi: {}, cos_theta: {}", mat.reflectance(), 1.0/PI, cos_theta);
            // let color = color::mul_s(&mat.reflectance(&ray.dir, &new_ray.dir, &sp.normal), k);
            // //let color = mat.reflectance();
            // let reflected = self.trace_path_rec::<S, C>(scene, &new_ray, depth - 1);
            // //println!("brdf: {:?}, reflected: {:?}", color, reflected);
            // return color::sum(&le, &color::mul_v(&reflected, &color));
            let shifted_ray_pos = sp.position + sp.normal * consts::RAY_SHIFT_DISTANCE;
            //let (mut new_ray, reflectance) = mat.brdf(&ray.dir, &sp.position, &sp.normal);
            let (mut new_ray, reflectance) = mat.brdf(&ray.dir, &shifted_ray_pos, &sp.normal);
            let li: Color = self.trace_path_rec::<S, C>(scene, &new_ray, depth - 1);

            return color::sum(&le, &color::mul_v(&reflectance, &li));


        } else {
            //Color {data: [1.0, 0.5, 0.5, 1.0f32]}
            //return color::BLACK;
            return color::BLACK;
        }

    
    }


//     fn trace_path_rec<S, C>(&mut self, scene: &S, ray: &Ray3f, depth: u32) -> Color
//         where S: SceneHolder, C: RenderCamera
//     {
//         if depth == 0 {
//             return color::BLACK;
//         }

//         if let Some(sp) = scene.intersection_with_scene(ray) {
//             let mat = sp.material;

//             let Closed01(rnd) = rand::random::<Closed01<f32>>();
//             let pe = 0.5f32;
//             let ip = 1.0 / pe;
//             if rnd < pe { // emmited
//                 let le = if let Some(c) = mat.emission() {
//                     c
//                 } else {
//                     color::BLACK
//                     //color::GREEN
//                 };
//                 return color::mul_s(&le, ip);

//             } else { // reflected

//                 let new_ray = mat.reflect_ray(&ray.dir, &sp.position, &sp.normal);
//                 let cos_theta = new_ray.dir.dot(&sp.normal);                
        
//                 //let k =  2.0 * ip * cos_theta;       
//                 let k = ip;

//                 let r = self.trace_path_rec::<S, C>(scene, &new_ray, depth - 1);
//                 let m = mat.reflectance();
//                 return  color::mul_s(&color::mul_v(&r, &m), k);

//             }

//         } else {
//             return color::BLACK;
//             //return color::RED;
//         }

//     }


}

impl<S: SceneHolder, C: RenderCamera> RendererHelper<S, C> for PathTracer {
    fn trace_path(&self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color {
        let mut res = self.trace_path_rec::<S, C>(scene, &initial_ray, setup.path_depth);

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