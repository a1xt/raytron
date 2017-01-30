/*

TODO:
- Rendering framework (init gapi, input control, gui, debug render);
- Debug module (drawSphre, drawLine, drawCube, drawMesh);
- Path tracing algorithms:
  - backward pt
  - forward pt
  - fast bdpt
  - bdpt
  - mlt?
  - volume render(fog only)
  - refraction depth stack?
- Add support to primitives (sphere, triangle, ?voxel(no));
- Materials (.mtl, phys-based?(no));
- Scene holders (kd-tree, bvh(no));
- Loading models from file (.obj, .mtl);

*/


#[macro_use]
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate glium;

use std::fs::File;
use std::path::Path;


use image::{
    RgbImage,
    Rgb,
    Pixel,
};

use glium::{
    DisplayBuild,
    Surface,
};


use cgmath::*;


use test::*;
mod test;
mod gfx;


#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex! (Vertex, pos, tex_coords);


fn main() {
   // let img = ImageBuffer::new(800, 600);

    let scene = Scene {
        prims: vec![
            Primitive::Sphere {pos: Vector3::from((0.0, 0.0, 2.0)), radius: 0.25},
            Primitive::Sphere {pos: Vector3::from((3000.0, 0.0, 8000.0)), radius: 1000.0},
            Primitive::Sphere {pos: Vector3::from((0.0, -0.65, 2.0)), radius: 0.25},
        ],
        ambient_light: Rgb::from_channels(0,0,128,0),
    };
    let cam = Camera {
        width: 1024,
        height: 1024,
        transform: Matrix4::zero(),
    };

    let mut img = RgbImage::new(1024,1024);

    render_scene(&cam, &scene, &mut img);


    let path = Path::new("result.png");
    let _ = File::create(&path).unwrap();

    let _ = img.save(path).unwrap();

    
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let img_dims = img.dimensions();
    let tex_src = glium::texture::RawImage2d::from_raw_rgb(img.into_raw(), img_dims);
    let tex = glium::texture::Texture2d::new(&display, tex_src).unwrap();

    let fullscreen_quad = vec! [
        Vertex {pos: [-1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {pos: [-1.0,  1.0], tex_coords: [0.0, 1.0]},
        Vertex {pos: [ 1.0,  1.0], tex_coords: [1.0, 1.0]},
        Vertex {pos: [ 1.0, -1.0], tex_coords: [1.0, 0.0]},
    ];
    let indices: Vec<u32> = vec! [
        0, 1, 2,
        0, 2, 3,
    ];
    let vshader_src = r#"
        #version 140

        in vec2 pos;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;
    let fshader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;
    let vb = glium::VertexBuffer::new(&display, &fullscreen_quad).unwrap();
    let ib = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
    //let ib = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let prog = glium::Program::from_source(&display, &vshader_src, &fshader_src, None).unwrap();
    let uniforms = uniform! {
        tex: &tex,
    };

    

    let render = | | {
        let mut render_target = display.draw();
        render_target.clear_color(0.0, 1.0, 1.0, 1.0);
        render_target.draw(&vb, &ib, &prog, &uniforms, &Default::default()).unwrap();
        render_target.finish().unwrap();
    };

    loop {
        for e in display.poll_events() {
            match e {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
        render();
    }
}