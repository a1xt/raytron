#![feature(box_syntax)]
#![feature(type_ascription)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
pub mod common;
use common::*;
use image::hdr;
use rtcore::{Image, Mesh, Polygon, RenderSettings, TexView, Texture};
use rtcore::bsdf::{Bsdf, CookTorrance, Diffuse, Phong};
use rtcore::bsdf::cooktorrance::*;
use rtcore::color;
use rtcore::color::{Color, Luma, Rgb};
use rtcore::material::{DiffuseMat, DiffuseTex, PbrTex};
use rtcore::math;
use rtcore::math::{Norm, Point2, Point3f, Real, Vector2, Vector3, Vector3f};
use rtcore::renderer::PathTracer;
use rtcore::scenehandler::{KdTreeS, ShapeList};

use rtcore::scenehandler::{LinearSampler, ShapeListBuilder, UniformSampler};
use rtcore::scenehandler::kdtree::{KdTreeSetup, Sah};
use rtcore::sphere::Sphere;
use rtcore::traits::{BoundedSurface, Surface};
use rtcore::traits::{Renderer, SceneHandler};
use rtcore::vertex::{BaseVertex, TexturedVertex};
use scenes::{Cube, CubeSide, Plane, Quad};

use scenes::spheres;
use std::collections::BTreeMap;
use std::iter::once;
use std::sync::Arc;

pub fn lifetime_hack<'a, 'b, T>(t: &'a T) -> &'b T {
    unsafe { ::std::mem::transmute(t) }
}

pub struct Materials {}

impl Materials {
    fn new() -> Self {
        Self {}
    }

    fn add_spheres<'a, F>(&self, mut add_shape: F)
    where
        F: FnMut(Box<Surface + 'a>),
    {
        let spheres_num = 5;
        let radius = 65.0;
        let offset_x = 15.0;
        let offset_y = 50.0;
        let pos_z = 0.0;
        let roughness_min = 0.05;
        let roughness_max = 1.0;
        let rows_num = 3;

        let mut pos_y =
            ((rows_num - 1) as Real) * radius + offset_y * (rows_num / 2) as Real;
        for r in 0..rows_num {
            let mut pos_x =
                0.0 - ((spheres_num - 1) as Real) * radius - offset_x * (spheres_num / 2) as Real;
            for i in 0..spheres_num {
                let roughness = roughness_min +
                    (roughness_max - roughness_min) * (i as Real / (spheres_num - 1) as Real);
                println!("roughness({}): {}", i, roughness);
                let mat: Arc<Bsdf> = if r == 0 {
                    Arc::new(CookTorrance::new(
                        color::BLACK,
                        color::WHITE * roughness as f32,
                        0.0,
                    ))
                } else if r == 1 {
                    Arc::new(CookTorrance::new(
                        color::WHITE,
                        color::BLACK,
                        roughness * roughness,
                    ))
                } else {
                    //let mut c: Rgb<f32> = color::Rgb::<u8>::new(212, 175, 55).into();
                    //Arc::new(CookTorrance::new(color::BLACK, c, roughness * roughness))
                    Arc::new(CookTorrance::new(
                        color::BLACK,
                        color::WHITE,
                        roughness * roughness,
                    ))
                };
                let sphere = box Sphere::new(Point3f::new(pos_x, pos_y, pos_z), radius, mat);

                pos_x += 2.0 * radius + offset_x;

                add_shape(sphere as Box<Surface>);
            }
            pos_y -= 2.0 * radius + offset_y;
        }
    }

    fn add_plane<'a, F>(&self, mut add_shape: F)
    where
        F: FnMut(Box<Surface + 'a>),
    {
        let gen_tex_w = 32usize;
        let gen_tex_h = 32usize;
        let mut gen_tex: Texture<Rgb> = Texture::new(gen_tex_w, gen_tex_h);
        for j in 0..gen_tex.height() {
            for i in 0..gen_tex.width() {
                let black = (j % 2) ^ (i % 2) != 0;
                if black {
                    gen_tex.set_pixel(i, j, color::BLACK);
                } else {
                    gen_tex.set_pixel(i, j, color::WHITE);
                }
            }
        }

        let basecolor_tex: Texture<Rgb<f64>> = mono_texture!(color::WHITE.into());
        let roughness_tex: Texture<Luma<f64>> = mono_texture!(Luma::from(0.0));
        let spec_tex: Texture<Luma<f64>> = mono_texture!(Luma::from(1.0));
        let metal_tex: Texture<Luma<f64>> = mono_texture!(Luma::from(1.0));
        let mut normal_tex: Texture<Rgb<f64>> =
            load_texture_rgb::<f64>("data/testnormal.png".to_string(), false);
        for n in normal_tex.as_mut_slice() {
            let dx_n: Rgb<Real> = Rgb::from(*n);
            let gl_n = ::utils::normal_dx_to_ogl(&dx_n);
            *n = gl_n.into();
        }

        let plane_mat: Arc<PbrTex<Rgb<Real>, Luma<Real>>> = Arc::new(PbrTex::new(
            basecolor_tex,
            normal_tex,
            roughness_tex,
            spec_tex,
            metal_tex,
        ));

        // let tex = Arc::new(gen_tex) as Arc<TexView<Color>>;
        // let plane_mat = Arc::new(DiffuseTex::new(tex.clone(), Some(tex.clone())));

        // let path = "data/rusted_iron/".to_string();
        // let path = "data/rusted_iron2/".to_string();
        // let path = "data/cement/".to_string();
        // let plane_mat = load_pbr(path);
        // let plane_mat = Arc::new(DiffuseMat::new(color::WHITE, None));

        let plane_mesh = Plane::build(
            Point3f::new(0.0, 0.0, 0.0),
            Point3f::new(0.0, 0.0, 1.0),
            Vector3f::new(0.0, 1.0, 0.0),
            (700.0, 700.0),
            plane_mat,
            Some((2, 2)),
            None,
            |quad| {
                Quad {
                    v0: TexturedVertex::new(quad.v0, Vector2::new(0.0, 0.0)),
                    v1: TexturedVertex::new(quad.v1, Vector2::new(0.0, 1.0)),
                    v2: TexturedVertex::new(quad.v2, Vector2::new(1.0, 1.0)),
                    v3: TexturedVertex::new(quad.v3, Vector2::new(1.0, 0.0)),
                }
            });

        let plane_pols = plane_mesh.into_polygons();
        for p in plane_pols {
            add_shape(box p as Box<Surface>)
        }

    }
}

impl AppState for Materials {
    fn new() -> Self {
        Self::new()
    }

    fn init(&mut self) -> ExampleAppBuilder {
        ExampleAppBuilder::new().size(400, 300)
    }

    fn init_camera(&self, ctrl: &mut FPSCameraController) {
        ctrl.camera_mut().set_pos(&Point3f::new(0.0, 0.0, 800.0));
        ctrl.set_move_speed(50.0);
        ctrl.set_mouse_sensitivity(0.20);
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHandler + 's> {
        let mut scene = ShapeListBuilder::<_, UniformSampler>::new();
        let cube_size = 1000.0;
        let room_mesh = Cube::build(
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
                CubeSide::Top => Arc::new(DiffuseMat::new(color::WHITE, Some(color::WHITE))),
                CubeSide::Left => Arc::new(DiffuseMat::new(Color::new(0.25, 0.25, 0.75), None)),
                CubeSide::Right => Arc::new(DiffuseMat::new(Color::new(0.75, 0.25, 0.25), None)),
                _ => Arc::new(DiffuseMat::new(color::WHITE * 0.75, None)),
            },
            |_| (1, 1),
            true,
        );

        let room_pols = room_mesh.into_polygons();

        for p in room_pols {
            scene.add_shape((box p) as Box<Surface>);
        }

        {
            let scene_ref = &mut scene;
            self.add_spheres(move |s| scene_ref.add_shape(s));
        }

        // {
        //     let scene_ref = &mut scene;
        //     self.add_plane(move |s| scene_ref.add_shape(s));
        // }

        box scene.into_shape_list()
    }

    fn post_process(&self, img: &mut Image) {
        let t = 1.125;
        //tone_mapping_exp(img, t);
        gamma_encoding(img);
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
