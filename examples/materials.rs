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
use pt::bsdf::{Phong, Diffuse, CookTorrance};
use pt::sphere::Sphere;
use pt::color;
use pt::color::{Color, Rgb};
use image::hdr;
use scenes::{Cube, Quad, CubeSide};
use pt::material::{DiffuseTex, DiffuseMat};
use pt::vertex::{BaseVertex, TexturedVertex};
use std::sync::Arc;
use std::collections::BTreeMap;
use pt::math::{Point3f, Point2, Vector3f};

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

        let roughness = 0.7;
        let sphere = box Sphere::new(
            Point3f::new(0.0, 0.0, 0.0), 
            100.0,
            //Arc::new(Diffuse::new(color::BLACK, None)));
            Arc::new(CookTorrance::new(color::BLACK, Vector3f::new(0.2, 0.2, 0.2), roughness * roughness)));
            //Arc::new(Phong::new(color::WHITE, 0.0, 1.0, 10000.0)));

        scene.add_shape(sphere);
        
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