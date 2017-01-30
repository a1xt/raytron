use ::{RenderSettings, Color};
use math::{Ray3f, Dot, Norm};
use color;
use traits::{Renderer, SceneHolder, RenderCamera};
use std::f32::consts::PI;
use rand::{Closed01};
use rand;


pub struct DbgRayCaster {

}

impl DbgRayCaster {
    pub fn new () -> DbgRayCaster {
        DbgRayCaster { }
    }

    fn trace_path_rec<S, C>(&mut self, scene: &S, ray: &Ray3f, depth: u32) -> Color
        where S: SceneHolder, C: RenderCamera
    {

        if let Some(sp) = scene.intersection_with_scene(ray) {
            let mat = sp.material;
            if let Some(c) = mat.emission() {
                return c;
            } else {
                if let Some(light) = scene.random_light_source() {
                    let light_point = light.random_point();

                    let mut shadow_ray = Ray3f::new(sp.position, (light_point.position - sp.position).normalize());
                    shadow_ray.origin += sp.normal * 0.1;
                    

                    //return Color{data: [sp.normal.x.abs(), sp.normal.y.abs(), sp.normal.z.abs(), 1.0]}; 


                    if let Some(lp) = scene.intersection_with_scene(&shadow_ray){
                        
                        if let Some(_) = lp.material.emission() {
                            let cos_theta = sp.normal.dot(&shadow_ray.dir);

                            return color::mul_s(&color::WHITE, cos_theta);
                        }
                         else {
                            if (lp.normal.dot(&shadow_ray.dir) > 0.0) {
                               return color::RED;
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

impl<S: SceneHolder, C: RenderCamera> Renderer<S, C> for DbgRayCaster {
    fn trace_path(&mut self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color {
        let mut res = self.trace_path_rec::<S, C>(scene, &initial_ray, setup.path_depth);

        res        
    }
}