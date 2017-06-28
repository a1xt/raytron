#![feature(box_syntax)]
#![feature(type_ascription)]
#![allow(unused_imports)]

pub mod common;
use common::*;

use scenes::spheres;
use pt::traits::{SceneHandler, Renderer};
use pt::renderer::PathTracer;
use pt::scenehandler::{ShapeList, KdTreeS};
use pt::{Image, Texture, Tex, Mesh, Polygon, RenderSettings};
use pt::bsdf::{Bsdf, Phong, Diffuse, CookTorrance};
use pt::bsdf::cooktorrance::*;
use pt::sphere::Sphere;
use pt::color;
use pt::color::{Color, Rgb};
use image::hdr;
use scenes::{Cube, Quad, CubeSide};
use pt::material::{DiffuseTex, DiffuseMat};
use pt::vertex::{BaseVertex, TexturedVertex};
use std::sync::Arc;
use std::collections::BTreeMap;
use pt::math::{Real, Point3f, Point2, Vector3f};
use pt::math;

use pt::scenehandler::{ShapeListBuilder, UniformSampler, LinearSampler};
use pt::scenehandler::kdtree::{KdTreeSetup, Sah};
use pt::traits::{BoundedSurface, Surface};
use std::iter::once;

pub fn lifetime_hack<'a, 'b, T>(t: &'a T) -> &'b T {
    unsafe {::std::mem::transmute(t) }
}

pub struct Materials {

}

impl Materials {
    fn new() -> Self {
        Self {
        }
    }
}

impl AppState for Materials {
    fn new() -> Self {
        let mut s = Self::new();
        s
    }

    fn init(&mut self) -> ExampleAppBuilder {
        let builder = ExampleAppBuilder::new().size(400, 300);
        builder
    }

    fn init_camera(&self, ctrl: &mut FPSCameraController) {
        ctrl.camera_mut().set_pos(&Point3f::new(0.0, 0.0, 400.0));
        ctrl.set_move_speed(50.0);
        ctrl.set_mouse_sensitivity(0.20);
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHandler + 's> {
        let mut scene = ShapeListBuilder::<_, UniformSampler>::new();
        let cube_size = 1000.0;
        let room_mesh = box Cube::build(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::new(cube_size, cube_size, cube_size),
            |_, quad| {
                Quad {
                    v0: BaseVertex::new(quad.v0),
                    v1: BaseVertex::new(quad.v1),
                    v2: BaseVertex::new(quad.v2),
                    v3: BaseVertex::new(quad.v3),         
                }
            },
            |side| {
                match side {
                    CubeSide::Top => Arc::new(DiffuseMat::new(color::WHITE, Some(color::WHITE))),
                    CubeSide::Left => Arc::new(DiffuseMat::new(Color::new(0.25, 0.25, 0.75), None)),
                    CubeSide::Right => Arc::new(DiffuseMat::new(Color::new(0.75, 0.25, 0.25), None)),
                    _ => Arc::new(DiffuseMat::new(color::WHITE * 0.75, None)),
                }
            },
            |_| (1, 1),
            true);

        let room_pols = room_mesh.to_polygons();

        for p in room_pols {
            scene.add_shape((box p) as Box<Surface>);
        }

        let spheres_num = 5;
        let radius = 50.0;
        let offset_x = 15.0;
        let offset_y = 50.0;
        let roughness_min = 0.05;
        let roughness_max = 1.0;
        let row_ior = [
            Rgb::new(0.16761, 0.14462, 0.13536), // silver
            //Vector3f::new(2.5355, 2.0745, 1.8131), // platinum
            Rgb::new(0.16909, 0.44433, 1.4532),  // gold
            Rgb::new(0.21258, 0.70391, 1.3370)]; // copper

        let air_ior = Rgb::new(1.0, 1.0, 1.0);

        let mut pos_y =  ((row_ior.len() - 1) as Real) * radius + offset_y * (row_ior.len() / 2) as Real;
        for r in 0..row_ior.len() {
            let mut pos_x = 0.0 - ((spheres_num - 1) as Real) * radius - offset_x * (spheres_num / 2) as Real;
            for i in 0..spheres_num {
                let roughness = roughness_min + (roughness_max - roughness_min) * (i as Real / (spheres_num - 1) as Real);
                println!("roughness({}): {}",i, roughness);
                let mat: Arc<Bsdf> = if r == 0 {
                    Arc::new(Diffuse::new(color::WHITE, None))
                   // Arc::new(CookTorrance::new(color::BLACK, math::calc3_f0(&air_ior, &row_ior[r]), roughness * roughness))
                    //Arc::new(Phong::new(color::WHITE, 0.0, 1.0, 10000.0))
                } else if r == 1 {
                    Arc::new(CookTorrance::new(
                        color::WHITE, 
                        color::BLACK,
                        roughness * roughness))
                } else {
                    //let c: Rgb<f64> = color::GOLD.into();
                    let mut c: Rgb<f32> = color::Rgb::<u8>::new(212, 175, 55).into(); // gold
                    //let mut c: Rgb<f32> = color::Rgb::<u8>::new(69, 55, 36).into(); // gold
                    //use color::ColorClamp;
                    //let c = Rgb::new(c.r.powf(2.2), c.g.powf(2.2), c.b.powf(2.2)).clamp();
                
                    //let c: Rgb<f32> = Rgb::from(math::calc3_f0(&Rgb::from(1.5), &Rgb::from(1.0)));
                    println!("c = {:?}", c);
                    Arc::new(CookTorrance::new(
                        color::BLACK, 
                        c,
                        roughness * roughness ))
                };
                let sphere = box Sphere::new(
                    Point3f::new(pos_x, pos_y, 0.0), 
                    radius,
                    mat);
                
                pos_x += 2.0 * radius + offset_x;

                scene.add_shape(sphere);
            }
            pos_y -= 2.0 * radius + offset_y;
        }
        
        box scene.to_shape_list()

    }

    fn post_process(&self, img: &mut Image) {
        let t = 1.125; 
        //tone_mapping_exp(img, t);
        gamma_encoding(img);
        //gamma_decoding(img);
    }

    fn create_renderer<'s>(&'s self) -> (Box<Renderer<SceneHandler + 's> + 's>, RenderSettings) {
        let pt_render_chunk = (80, 60);
        let rdr_setup = RenderSettings::new(128, 6).with_threads(4, pt_render_chunk);       
        let rdr = box PathTracer::new(&rdr_setup).with_direct_illumination(0.5, 0.5);
        (rdr, rdr_setup)
    }

    // fn update(&mut self) { }
    // fn need_update(&self) -> bool { false }
    
    // fn init_camera(&self, &mut FPSCameraController) { }
    // //fn update_camera(&self, &mut CameraController) { }
}

fn main() {
    let mut state = Materials::new();
    ExampleApp::launch(&mut state);
}