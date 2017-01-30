#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate nalgebra as na;
extern crate genmesh;
extern crate pt_app as pt;
extern crate obj;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

mod consts;
mod camera;
// use camera::*;

use pt::utils::to_rad;

use std::ops::{Mul, Neg};
use na::{Isometry3, Matrix, Matrix4, Norm, Point3, ToHomogeneous, Vector3, Vector4};

use genmesh::{Triangulate, Vertices};
use genmesh::generators::{IndexedPolygon, SharedVertex, SphereUV};

use pt::camera_controller::*;
use pt::utils::camera::RenderCamera;

use std::fs::File;
use std::io::BufReader;

use glutin::CursorState;



gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant Locals {
        view: [[f32; 4]; 4] = "view",
        proj: [[f32; 4]; 4] = "proj",
        model: [[f32; 4]; 4] = "model",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        proj: gfx::Global<[[f32; 4]; 4]> = "proj",
        view: gfx::Global<[[f32; 4]; 4]> = "view",
        model: gfx::Global<[[f32; 4]; 4]> = "model",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,

    }
}
// const TRIANGLE: [Vertex; 3] = [
// Vertex { pos: [ -0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
// Vertex { pos: [  0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0, 1.0] },
// Vertex { pos: [  0.0,  0.5, 0.0], color: [0.0, 0.0, 1.0, 1.0] }
// ];
//
const TRIANGLE: [Vertex; 3] = [Vertex {
                                   pos: [-0.5, -0.5, 0.0],
                                   color: [1.0, 0.0, 0.0, 1.0],
                               },
                               Vertex {
                                   pos: [0.5, -0.5, 0.0],
                                   color: [0.0, 1.0, 0.0, 1.0],
                               },
                               Vertex {
                                   pos: [0.0, 0.366, 0.0],
                                   color: [0.0, 0.0, 1.0, 1.0],
                               }];
const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    // let pso = factory.create_pipeline_simple(include_bytes!("debug_140.glslv"), include_bytes!("debug_140.glslf"), pipe::new())
    //    .unwrap();
    let shaders = factory.create_shader_set(include_bytes!("debug_140.glslv"),
                           include_bytes!("debug_140.glslf"))
        .unwrap();
    let pso = factory.create_pipeline_state(&shaders,
                               gfx::Primitive::TriangleList,
                               gfx::state::Rasterizer {
                                   front_face: gfx::state::FrontFace::Clockwise,
                                   cull_face: gfx::state::CullFace::Nothing,
                                   method: gfx::state::RasterMethod::Line(1),
                                   offset: None,
                                   samples: None,
                               },
                               pipe::new())
        .unwrap();


    // let sphere = SphereUV::new(10, 10);
    // let sphere_vert: Vec<Vertex> = sphere.shared_vertex_iter()
    // .map(|(x, y, z)| {
    // Vertex {
    // pos: [x, y, z],
    // color: [1.0, 0.0, 0.0, 1.0],
    // }
    // })
    // .collect();
    //
    // let shpere_indices: Vec<u32> = sphere.indexed_polygon_iter()
    // .triangulate()
    // .vertices()
    // .map(|i| i as u32)
    // .collect();
    //
    let f = File::open("data\\model.obj").unwrap();
    let model = obj::load_obj(BufReader::new(f)).unwrap();
    let vertx: Vec<Vertex> = model.vertices
        .into_iter()
        .map(|v: obj::Vertex| {
            Vertex {
                pos: v.position,
                color: [1.0, 0.0, 0.0, 1.0],
            }
        })
        .collect();
    let indx: Vec<u32> = model.indices.into_iter().map(|i| i as u32).collect();

    // let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    // let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&sphere_vert, &shpere_indices[..]);
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertx, &indx[..]);
    println!("loaded");

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out_color: main_color,
        out_depth: main_depth,
        view: {
            let i: Matrix4<f32> = na::Isometry3::look_at_rh(&Point3::new(0.0, 0.0, -1.0),
                                                            &Point3::new(0.0, 0.0, 1.0),
                                                            &Vector3::new(0.0, 1.0, 0.0))
                .to_homogeneous();
            *i.as_ref()
        },
        model: {
            let i: Matrix4<f32> = na::one();
            *i.as_ref()

        },
        proj: {
            let i: Matrix4<f32> = na::PerspectiveMatrix3::new(1024.0 / 768.0,
                                                              0.5 * std::f32::consts::PI,
                                                              0.901,
                                                              100000.0)
                .to_matrix();
            *i.as_ref()

        },
    };

    let width = 1024u32;
    let height = 768u32;


    let mut cam =
        pt::utils::camera::FPSCamera::new(1024, 768, 0.5 * std::f32::consts::PI, 0.1, 1000.0);
    cam.set_pos(&Point3::new(0.0, 0.0, -1.0)).look_at(&Point3::new(0.0, 0.0, 1.0));

    let mut cam_ctlr = FPSCameraController::new(cam, 1.0, 100.0);


    window.set_cursor_state(glutin::CursorState::Grab).unwrap();

    let mut events: Vec<glutin::Event> = Vec::new();

    let mut set_cur_pos = |x: i32, y: i32| { 
                window.set_cursor_position(x, y).unwrap_or(());
            };
    

    'main: loop {
        events.clear();
        for event in window.poll_events() {
            events.push(event);
        }

        // loop over events
        for event in events.iter() {
            use glutin::{ElementState, Event, VirtualKeyCode};
            match *event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,

                _ => {}
            }
        }

        cam_ctlr.on_frame(
            events.iter(),
            &mut set_cur_pos ,
            &mut|lock| {
                window.set_cursor_state(
                    if lock {
                        CursorState::Grab
                    } else {
                        CursorState::Normal
                    }
                ).unwrap_or(());
            }
        );

        data.view = *cam_ctlr.camera().view_matrix().as_ref();

        // draw a frame
        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}