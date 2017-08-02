#![feature(box_syntax)]
#![feature(type_ascription)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod common;
use common::*;
use image::hdr;
use rtcore::{Image, Mesh, Polygon, PolygonS, RenderSettings, TexView, Texture};
use rtcore::bsdf::{CookTorrance, Diffuse, Phong};
use rtcore::color;
use rtcore::color::{Color, Rgb};
use rtcore::material::{DiffuseMat, DiffuseTex};
use rtcore::math;
use rtcore::math::{Point3f, Real, Vector2, Vector3f};
use rtcore::renderer::PathTracer;
use rtcore::scenehandler::{KdTreeS, ShapeList};
use rtcore::scenehandler::{LinearSampler, ShapeListBuilder, UniformSampler};
use rtcore::scenehandler::kdtree::{KdTreeSetup, Sah};
use rtcore::sphere::Sphere;
use rtcore::traits::{BoundedSurface, Surface};
use rtcore::traits::{Renderer, SceneHandler};
use rtcore::vertex::{TbnVertex, TexturedVertex};
use scenes::{Cube, CubeSide, Quad};

use scenes::spheres;
use std::collections::BTreeMap;
use std::io::Write;
use std::iter::once;
use std::sync::Arc;

pub fn lifetime_hack<'a, 'b, T>(t: &'a T) -> &'b T {
    unsafe { ::std::mem::transmute(t) }
}

pub struct Envmap {
    // hdr_img: Box<Image>,
    // black_tex: Box<Image>,
    //envbox: Cube<'static, TexturedVertex>,
    envbox_mesh: Box<Mesh<'static, TexturedVertex>>,
    envbox_polygons: Vec<Polygon<'static, TexturedVertex>>,
    model_polygons: Vec<PolygonS<'static, TbnVertex>>,
    sphere: Sphere,
}

impl Envmap {
    fn new() -> Self {
        // let hdr_img_path = "data/hdr/grace_cross.hdr".to_string();
        let hdr_img_path = "data/hdr/rnl_cross.hdr".to_string();
        // let hdr_img_path = "data/hdr/campus_cross.hdr".to_string();
        let hdr_img: Arc<TexView<Color>> = Arc::new(load_hdr(hdr_img_path));
        let black_tex: Arc<TexView<Color>> = Arc::new(Image::new(1, 1));
        let mat = Arc::new(DiffuseTex::new(black_tex, Some(hdr_img)));
        let cube_size = 1000.0;

        let f03 = 1.0 / 3.0;
        let f06 = 2.0 / 3.0;
        let mut tex_uv = BTreeMap::new();
        tex_uv.insert(
            CubeSide::Top,
            [(f03, 0.75), (f03, 1.0), (f06, 1.0), (f06, 0.75)],
        );
        tex_uv.insert(
            CubeSide::Bottom,
            [(f03, 0.25), (f03, 0.5), (f06, 0.5), (f06, 0.25)],
        );
        tex_uv.insert(
            CubeSide::Right,
            [(f06, 0.5), (f06, 0.75), (1.0, 0.75), (1.0, 0.5)],
        );
        tex_uv.insert(
            CubeSide::Left,
            [(0.0, 0.5), (0.0, 0.75), (f03, 0.75), (f03, 0.5)],
        );
        tex_uv.insert(
            CubeSide::Front,
            [(f03, 0.5), (f03, 0.75), (f06, 0.75), (f06, 0.5)],
        );
        tex_uv.insert(
            CubeSide::Back,
            [(f06, 0.25), (f06, 0.0), (f03, 0.0), (f03, 0.25)],
        );

        let envbox_mesh = box Cube::build(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::new(cube_size, cube_size, cube_size),
            |side, quad| {
                let uv = tex_uv[&side];
                Quad {
                    v0: TexturedVertex::new(quad.v0, Vector2::new(uv[0].0, uv[0].1)),
                    v1: TexturedVertex::new(quad.v1, Vector2::new(uv[1].0, uv[1].1)),
                    v2: TexturedVertex::new(quad.v2, Vector2::new(uv[2].0, uv[2].1)),
                    v3: TexturedVertex::new(quad.v3, Vector2::new(uv[3].0, uv[3].1)),
                }
            },
            |_| mat.clone(),
            |_| (50, 50),
            true,
        );

        print!("creating envbox polygons ...");
        let _ = std::io::stdout().flush();
        let envbox_polygons = unsafe { ::std::mem::transmute(envbox_mesh.to_polygons()) };
        println!("done!");

        // let model_mat = load_pbr::<Real>("data/rusted_iron2/".to_string(), false);
        // //let model_mat = load_pbr("data/rusted_iron/".to_string());
        // //let model_mat = Arc::new(DiffuseMat::new(color::WHITE, None));
        // let teapot_scale = 0.4;
        // //let scale = Vector3f::new(15.0, 15.0, 15.0);
        // let teapot_dpos = Vector3f::new(0.0, -20.0, 0.0);
        // let model_mesh = load_obj_pbr(
        //     "data/teapot.obj".to_string(),
        //     //"data/artorias/artorias.obj".to_string(),
        //     |_| model_mat.clone(),
        //     |pos| pos * teapot_scale + teapot_dpos,
        // ).into_iter()
        //     .fold(Mesh::new(), |mut base, mut mesh| {
        //         base.merge(&mut mesh);
        //         base
        //     });

        let scale = 0.03;
        // let scale = 5.0;
        let dpos = Vector3f::new(0.0, -50.0, 0.0);
        let model_mesh = load_obj_pbr(
            "data/artorias/artorias_knight.obj".to_string(),
            // "data/artorias/artorias.obj".to_string(),
            |name| load_pbr::<u8>("data/artorias/low/".to_string() + &name + "/", true),
            |pos| pos * scale + dpos,
        ).into_iter()
            .fold(Mesh::new(), |mut base, mut mesh| {
                base.merge(&mut mesh);
                base
            });




        let model_polygons = model_mesh.into_polygons();


        Self {
            // hdr_img,
            // black_tex,
            envbox_mesh,
            envbox_polygons,
            sphere: Sphere::new(
                Point3f::new(0.0, 0.0, 0.0),
                15.0,
                //Arc::new(Diffuse::new(color::WHITE, None)),
                //Arc::new(Phong::new(color::WHITE, 0.0, 1.0, 10000.0)),
                Arc::new(CookTorrance::new(color::WHITE, color::BLACK, 0.0)),
            ),
            model_polygons,
        }
    }

    fn init(&mut self) {
        //self.envbox_polygons = self.envbox_mesh.polygons();
    }
}

impl AppState for Envmap {
    fn new() -> Self {
        Self::new()
    }

    fn init(&mut self) -> ExampleAppBuilder {
        self.init();
        ExampleAppBuilder::new().size(200, 300)
    }

    fn init_camera(&self, ctrl: &mut FPSCameraController) {
        ctrl.camera_mut().set_pos(&Point3f::new(0.0, -10.0, 100.0));
        //ctrl.camera_mut().yaw_add((-30.0 as Real).to_radians());
        ctrl.set_move_speed(15.0);
        ctrl.set_mouse_sensitivity(0.20);
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHandler + 's> {
        // let mut scene = ShapeListBuilder::<&Surface, UniformSampler>::new();
        // for p in self.envbox_polygons.iter() {
        //     scene.add_shape(p.as_ref());
        // }
        // scene.add_shape(&self.sphere);
        // box scene.into_shape_list()

        let pol_iter = self.envbox_polygons.iter().map(|r| r as &BoundedSurface);
        //let it = pol_iter.chain(once(&self.sphere as &BoundedSurface));
        let model_iter = self.model_polygons.iter().map(|r| r as &BoundedSurface);
        let it = pol_iter.chain(model_iter);
        let kdtree_setup = KdTreeSetup::new(32, 16, Sah::new(1.0, 1.0));

        print!("building kd-tree ...");
        let _ = std::io::stdout().flush();
        let kdtree = box KdTreeS::<BoundedSurface, LinearSampler>::new(it, kdtree_setup);
        println!("done! (depth: {})", kdtree.depth());
        kdtree
    }

    fn post_process(&self, img: &mut Image) {
        let t = 0.75;
        // tone_mapping_exp(img, t);
        // tone_mapping_simple(img);
        gamma_encoding(img);
    }

    fn create_renderer<'s>(&'s self) -> (Box<Renderer<SceneHandler + 's> + 's>, RenderSettings) {
        let pt_render_chunk = (50, 60);
        let rdr_setup = RenderSettings::new(128, 2).with_threads(4, pt_render_chunk);
        let rdr = box PathTracer::new(&rdr_setup).with_direct_illumination(0.5, 0.5);
        (rdr, rdr_setup)
    }
}

fn main() {
    let mut state = Envmap::new();
    ExampleApp::launch(&mut state);
}
