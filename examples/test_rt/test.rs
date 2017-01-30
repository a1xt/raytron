extern crate cgmath;
extern crate image;

use image::{Rgb, Pixel, RgbImage};

use cgmath::*;
use std::option::*;

use std::f32;


#[derive(Clone, Copy)]
pub enum Primitive {
    Sphere {pos :Vector3<f32>, radius :f32},
    //Triangle {v0 :Vector3<f32>, v1 :Vector3<f32>, v2 :Vector3<f32>},

    #[allow(dead_code)]
    Unknown,
}

pub struct Scene {
    pub prims :Vec<Primitive>,
    pub ambient_light :Rgb<u8>,
}

pub type Color = Vector3<u8>;

pub struct Ray {
    origin :Vector3<f32>,
    normal :Vector3<f32>,
}
/*
pub struct Texture {
    height :u32,
    width :u32,
    data :Vec<Color>
}*/

pub type Texture = RgbImage;

pub struct Camera {
    pub width :u32,
    pub height :u32,
    pub transform :Matrix4<f32>
}

pub fn render_scene<'s, 'c, 'r> (camera: &'c Camera, scene: &'s Scene, result: &'r mut Texture) {
    assert!(camera.width == result.width() && camera.height == result.height(), "Unacceptable output texture format");

/*    let rwidth = (result.width() / 2) as f32;
    let rheight = (result.height() / 2) as f32;
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        let mut ray = Ray {
            origin: Vector3::zero(), 
            normal: Vector3 {
                x: x as f32 - rwidth, 
                y: y as f32 - rheight, 
                z: rwidth * 0.5f32
            }.normalize(),
        };
        //ray.normal = ray.normal.normalize();
        *pixel = raycast(scene, &ray);
    }*/

    let up = Vector3::from((0.0f32, 1.0, 0.0));
    let right = Vector3::from((1.0f32, 0.0, 0.0));
    let dir = Vector3::from((0.0f32, 0.0, 1.0));
    let pos = Vector3::zero();

    let rwidth = result.width() as f32;
    let rheight = result.height() as f32;
    
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        let norm_x = x as f32 / rwidth - 0.5f32;
        let norm_y = y as f32 / rheight - 0.5f32;
        let image_point = norm_x * right + norm_y * up + pos + dir;
        let ray_dir = image_point - pos;
        let ray = Ray {
            origin: pos,
            normal: ray_dir.normalize(),
        };
        *pixel = raycast(scene, &ray);
    }
}



pub fn raycast<'s>  (scene: &'s Scene, ray: &Ray) -> Rgb<u8> {
    let mut min_t = f32::MAX;
    for prim in scene.prims.as_slice() {
        if let Some(t) = find_intersection(ray, *prim) {
            min_t = min_t.min(t);
        }
    }
    if min_t < (f32::MAX) {
        //println!("min_t = {}", min_t);
        Rgb::from_channels(255,255,255,255)
    } else {
        scene.ambient_light
    }

   
}

pub fn find_intersection (ray: &Ray, prim: Primitive) -> Option<f32> {
    match prim {
        Primitive::Sphere {.. } => find_intersection_sphere(ray, prim),
        _ => None
    }
}

pub fn find_intersection_sphere (ray: &Ray, prim: Primitive) -> Option<f32> {
    if let Primitive::Sphere {pos: sphere_pos,  radius} = prim {
        let Ray {origin: ray_pos, normal: ray_norm} = *ray;

        let l = ray_pos - sphere_pos;
        let b = l.dot(ray_norm);
        let c = l.dot(l) - radius * radius;
        let d2 = b*b - c;
        if d2 >= 0.0 {
            let d = d2.sqrt();
            let t1 = -b + d;
            let t2 = -b - d;
            let t = t1.min(t2);

            //return Some(ray_pos + t * ray_norm);
            Some(t)
        } else {
            None
        }    

    } else {
        None
    }

}

/*
trait Viewport {
    fn height () -> u32;
    fn width () -> u32;

}

trait RayGen {

}

struct Camera {

}

struct Ray {
    pos,
    dir,
    wave_len,
    power,
    generation,
}

fn gen_rays (camera, scene) ->  image;
fn cast_ray (scene, ray) -> Option<point>;
fn gen_reflextion_ray (primitive, point) -> ray | [ray];
fn gen_refraction_ray (primitive, point) -> ray | [ray];
fn shading (primitive, material, intersection_point, ambient_light, scene);
fn raypath_to_color (ray_path) -> color;
*/