#![feature(box_syntax)]
#![feature(type_ascription)]

pub mod common;
use common::*;
use rtcore::{Image, PolygonS, RenderSettings};
use rtcore::color;
use rtcore::color::Rgb;
use rtcore::bsdf::{Diffuse};
use rtcore::material::{DiffuseMat};
use rtcore::math::{Point3f, Vector3f};
use rtcore::renderer::PathTracer;
use rtcore::sphere::Sphere;
use rtcore::traits::{Renderer, SceneHandler};
use rtcore::vertex::{BaseVertex};
use scenes::{Cube, CubeSide, Quad};
use std::sync::Arc;

pub struct Spheres {
    room_polygons: Vec<PolygonS<'static, BaseVertex>>,
    spheres: Vec<Sphere>,
}

impl Spheres {
    fn new() -> Self {
        let cube_size = 200.0;
        let cube_mesh = box Cube::build(
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
            |side| match side {
                CubeSide::Left => Arc::new(DiffuseMat::new(
                    Rgb::new(0.25, 0.25, 0.75),
                    None
                )),
                CubeSide::Right => Arc::new(DiffuseMat::new(
                    Rgb::new(0.75, 0.25, 0.25),
                    None
                )),
                _ => Arc::new(DiffuseMat::new(
                    color::WHITE * 0.75,
                    None
                )),

            },
            |_| (1, 1),
            true,
        );

        let room_polygons = cube_mesh.into_polygons();

        let lpow = 15.0;
        let spheres = vec![
            Sphere::new(
                Point3f::new(-50.0, -70.0, -50.0),
                30.0,
                Arc::new(Diffuse::new(color::WHITE, None))),
            Sphere::new(
                Point3f::new(50.0, 60.0, -10.0),
                30.0,
                Arc::new(Diffuse::new(color::WHITE, None))),
            Sphere::new(
                Point3f::new(0.0, 0.0, -70.0),
                30.0,
                Arc::new(Diffuse::new(color::WHITE, None))),
            Sphere::new(
                Point3f::new(70.0, -70.0, 35.0),
                30.0,
                Arc::new(Diffuse::new(color::WHITE, None))),
            Sphere::new(
                Point3f::new(70.0, 0.0, 0.0),
                5.0,
                Arc::new(Diffuse::new(color::WHITE, Some(color::WHITE * lpow)))),
            Sphere::new(
                Point3f::new(50.0, 70.0, 90.0),
                5.0,
                Arc::new(Diffuse::new(color::WHITE, Some(color::YELLOW * lpow)))),
            Sphere::new(
                Point3f::new(-60.0, 0.0, -70.0),
                5.0,
                Arc::new(Diffuse::new(color::WHITE, Some(color::FUCHSIA * lpow)))),
            Sphere::new(
                Point3f::new(-60.0, -50.0, 90.0),
                5.0,
                Arc::new(Diffuse::new(color::WHITE, Some(color::CYAN * lpow)))),
        ];

        Self {
            room_polygons,
            spheres,
        }
    }
}

impl AppState for Spheres {
    fn new() -> Self {
        Self::new()
    }

    fn init(&mut self) -> ExampleAppBuilder {
        ExampleAppBuilder::new().size(512, 512)
    }

    fn init_camera(&self, ctrl: &mut FPSCameraController) {
        ctrl.camera_mut().set_pos(&Point3f::new(0.0, 0.0, 300.0));
        ctrl.set_move_speed(15.0);
        ctrl.set_mouse_sensitivity(0.20);
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHandler + 's> {
        use rtcore::scenehandler::{ShapeListBuilder};
        use rtcore::traits::{Surface};
        use rtcore::scenehandler::{LinearSampler};

        let mut scene = ShapeListBuilder::<_, LinearSampler>::new();
        for s in &self.room_polygons {
            scene.add_shape(s as &Surface);
        }
        for s in &self.spheres {
            scene.add_shape(s as &Surface);
        }

        box scene.into_shape_list()
    }

    fn post_process(&self, img: &mut Image) {
        let t = 0.95;
        tone_mapping_exp(img, t);
        // tone_mapping_simple(img);
        gamma_encoding(img);
    }

    fn create_renderer<'s>(&'s self) -> (Box<Renderer<SceneHandler + 's> + 's>, RenderSettings) {
        let pt_render_chunk = (64, 64);
        let rdr_setup = RenderSettings::new(128, 10).with_threads(4, pt_render_chunk);
        let rdr = box PathTracer::new(&rdr_setup).with_direct_illumination(0.15, 0.85);
        (rdr, rdr_setup)
    }
}

fn main() {
    let mut state = Spheres::new();
    ExampleApp::launch(&mut state);
}
