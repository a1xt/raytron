#![feature(box_syntax)]

extern crate pt_app;
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate glutin;
extern crate time;
extern crate image;
extern crate rand;

use pt_app::{App};
use pt_app::scenes::spheres;
use pt_app::pt::renderer::{PathTracer, DbgRayCaster};
use pt_app::pt::sceneholder::{ShapeList};
use pt_app::pt::{Image, RenderSettings};
use pt_app::pt::traits::{Renderer, BoundedSurface, SceneHolder};

use std::mem;

use gfx::format::R32_G32_B32_A32;
use gfx::format::Float;


//use pt_app::pt::rand;
use pt_app::pt::color;
use std::string::{String, ToString};

use glutin::{Event, ElementState, VirtualKeyCode};

use pt_app::scenes::meshes::Cube;
use pt_app::scenes::{cube_with_tex};
use pt_app::pt::mesh::{Mesh};
use pt_app::pt::polygon::{Polygon};

type TexFormat = Rgba32F;

use gfx::{Factory};
use gfx::format::{Rgba32F};

fn main () {
    let width = 400u32;
    let height = 300u32;

    //let pt_render_chunk = (64, 64);
    //let dbg_render_chunk = (128, 128);
    let pt_render_chunk = (80, 60);
    let dbg_render_chunk = (100, 75);

    let tex_w = width as u16;
    let tex_h = height as u16;

    let mut app = App::<gfx_gl::Device, gfx_gl::Factory>::new(width, height, "apptest".to_string());

    let texels = [[0xA0, 0x20, 0xC0, 0x00]];
    //let tf32 = [[0.7125, 0.6f32, 0.75]];//,  0.0]];
    let tf32 = [[1.0, 0.0f32, 0.0, 1.0], [0.5, 1.0f32, 1.0, 1.0], [1.0, 0.0f32, 0.5, 1.0], [0.0, 0.5f32, 1.0, 1.0]];
    //let tf32 = [[1.0, 0.0f32, 0.0, 1.0], [0.0, 1.0f32, 0.0, 1.0], [0.0, 0.0f32, 1.0, 1.0], [0.0, 0.0f32, 0.0, 1.0]];
    // let (tex0, texture_view) = app.factory_mut().create_texture_const::<gfx::format::Rgba32F>(
    //     gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single), &[&texels]
    //     ).unwrap();

    let tex = app.factory_mut().create_texture::<gfx::format::R32_G32_B32_A32> (
        gfx::texture::Kind::D2(tex_w, tex_h, gfx::texture::AaMode::Single),
        1,
        gfx::SHADER_RESOURCE,
        gfx::memory::Usage::Dynamic,
        Some(gfx::format::ChannelType::Float)
    ).unwrap();

    let tex_view = app.factory_mut().view_texture_as_shader_resource::<(R32_G32_B32_A32, Float)>(
        &tex,
        (0,0),
        gfx::format::Swizzle::new()
    ).unwrap();


    // RenderSettings::new(samples, depth);
    let setup = RenderSettings::new(128, 4).with_threads(4, pt_render_chunk);    
    let dbg_setup = RenderSettings::new(1, 1).with_threads(4, dbg_render_chunk);


    use pt_app::pt::math::{Point3f, Point2};
    use pt_app::pt::texture::Texture;
    use pt_app::pt::color::{self, Rgb, ColorBlend};
    use pt_app::pt::polygon::vertex::{TexturedVertex };
    use pt_app::pt::polygon::material::{DiffuseTex};
    use std::sync::Arc;

    let diftex_w = 128usize;
    let diftex_h = 128usize;
    let mut dif_tex: Texture<Rgb, [f32; 4]> = Texture::new(diftex_w, diftex_h);
    for j in 0..dif_tex.height() {
        for i in 0..dif_tex.width() {
            let u0v0 = color::YELLOW;
            let u0v1 = color::RED;
            let u1v1 = color::BLUE;
            let u1v0 = color::GREEN;
            let u = (i as f32) / (diftex_w - 1) as f32;
            let v = (j as f32) / (diftex_h - 1) as f32;
            let c = (u0v0 * (1.0 - u) + u1v0 * u) * (1.0 - v) + (u0v1 * (1.0 - u) + u1v1 * u) * v;
            dif_tex.set_pixel(i, j, c);
        }
    }

    // let verts = [
    //     TexturedVertex::new(Point3f::new(-30.0, -30.0, -30.0), Point2::new(0.0, 0.0)),
    //     TexturedVertex::new(Point3f::new(-30.0, 30.0, -30.0), Point2::new(0.0, 1.0)),
    //     TexturedVertex::new(Point3f::new(30.0, 30.0, -30.0), Point2::new(1.0, 1.0)),
    //     TexturedVertex::new(Point3f::new(30.0, -30.0, -30.0), Point2::new(1.0, 0.0)),
    // ];

    // let mat = Arc::new(DiffuseTex::new(&dif_tex));

    // let pol0 = Polygon::new(&verts[0], &verts[1], &verts[2], mat.clone());
    // let pol1 = Polygon::new(&verts[0], &verts[2], &verts[3], mat.clone());

    // let cube = Cube::with_bv(Point3f::new(0., 0., 0.,), (20., 20., 20.,));
    // let cube_mesh = cube.build_bv(false);
    // let polygons = cube_mesh.polygons();

    let cube_mesh = cube_with_tex(Point3f::new(0., 0., 0.), (20., 20., 20.), &dif_tex);
    let polygons = cube_mesh.polygons();

    //let (scene, _) = spheres::create_scene();
    let s = spheres::Room::new();
    let mut scene = Box::new(s.shape_list());

    // scene.add_shape(&pol0);
    // scene.add_shape(&pol1);

    for p in polygons.iter() {
        scene.add_shape(p);
    }

    // use pt_app::pt::sceneholder::kdtree::{KdTreeS, KdTreeSetup, Sah};
    // let obj_iter = s.shape_iter()
    //     .map(move |s| s as &BoundedSurface)
    //     .chain(
    //         polygons.iter()
    //                 .map(move |p| p as &BoundedSurface)
    //     );
    // println!("start building kdtree...");
    // let kdtree_build_start = time::precise_time_ns();
    // let kdtree_setup = KdTreeSetup::new(32, 8, Sah::new(1.0, 1.0));
    // let scene = KdTreeS::new(obj_iter, kdtree_setup);
    // let kdtree_build_time = time::precise_time_ns() - kdtree_build_start;
    // println!("kdtree builded, time: {:.2}", (kdtree_build_time as f64) * 1.0e-9);

    spheres::setup_camera(app.cam_ctrl_mut());
    let mut rdr = PathTracer::new(&setup).with_direct_illumination(0.75, 0.25);
    let x = app.cam_ctrl().camera().pos().x;
        let y = app.cam_ctrl().camera().pos().y;
        let z = app.cam_ctrl().camera().pos().z;
        println!("camera pos: ({:.1}, {:.1}, {:.1})", x, y, z);


    let mut dbg_rdr = DbgRayCaster::new();
    
    let mut img: Image = Image::new(width as usize, height as usize);
    
    
    //rdr.render_scene(&scene, app.cam_ctrl().camera(), &setup, &mut img);
    //println!("scene succesfully rendered!");

    //let image_name: String = "res_img/".to_string() + rand::random::<u32>().to_string().as_ref() + ".png";
    //println!("img_name: {}", image_name);
    // let mut buf: Vec<u8> = Vec::new();
    // for c in img.pixels() {
    //     //let cc = color::clamp_rgba(c);
    //     let cc = color::round_rgba(c);
    //     buf.push((cc[0] * 255.0) as u8);
    //     buf.push((cc[1] * 255.0) as u8);
    //     buf.push((cc[2] * 255.0) as u8);
    //     //buf.push((cc[3] * 256.0) as u8);
    // }

    //image::save_buffer(&std::path::Path::new(&image_name), buf.as_ref(), width, height, image::RGB(8)).unwrap();
    //println!("image saved!");

    let mut dbg = true;
    let mut pass_num = 0;
    let mut start_time;
    let mut total_time = 0u64;
    while app.run() {
        let scn = scene.as_ref() as &SceneHolder;
        if dbg {
            dbg_rdr.render_scene_threaded(scn, app.cam_ctrl().camera(), &dbg_setup, &mut img);
        } else {
            start_time = time::precise_time_ns();
            if pass_num == 0 {
                rdr.pre_render(scn, app.cam_ctrl().camera(), &setup);
                total_time = 0;
            }
            rdr.render_pass_threaded(scn, app.cam_ctrl().camera(), &setup, pass_num, &mut img);
            pass_num += 1;
            let frame_time = time::precise_time_ns() - start_time;
            total_time = total_time + frame_time;
            println!("pass_num: {}, frame time: {:.2}, average time: {:.2}", 
                pass_num, (frame_time as f64) * 1.0e-9, (total_time as f64) / (pass_num as f64) * 1.0e-9);
        }

        for event in app.events().iter() {
            match *event {
                Event::KeyboardInput(el_state, _, Some(key)) => {
                    let pressed = el_state == ElementState::Pressed;
                    match key {
                        VirtualKeyCode::R if pressed => {
                            if dbg {
                                dbg = false;
                                pass_num = 0;
                            } else {
                                dbg = true;
                            }
                        },

                        VirtualKeyCode::I if pressed => {
                            let image_name: String = "res_img/".to_string() + rand::random::<u16>().to_string().as_ref() + ".png";
                            println!("img_name: {}", image_name);

                            let mut buf: Vec<u8> = Vec::new();
                            for c in img.pixels() {
                                let cc: color::Rgb<u8> = c.into();
                                // let lum = color::Rgb::new(0.3, 0.59, 0.11);
                                // let l = lum.r * c.r + lum.g * c.g + lum.b * c.b;
                                // let s = l / (1.0 + l);
                                // let cc: color::Rgb<u8> = (c * s).into();
                                // let r = (c.r / (1.0 + c.r)) * 255.0;
                                // let g = (c.g / (1.0 + c.g)) * 255.0;
                                // let b = (c.b / (1.0 + c.b)) * 255.0;
                                // // buf.push(r as u8);
                                // // buf.push(g as u8);
                                // // buf.push(b as u8);
                                buf.push(cc.r);
                                buf.push(cc.g);
                                buf.push(cc.b);
                            }

                            image::save_buffer(&std::path::Path::new(&image_name), buf.as_ref(), width, height, image::RGB(8)).unwrap();
                            println!("image saved!");
                        },

                        _ => (),
                    }
                }

                _ => {}
            }
        }
        

        app.clear_frame();

        let _ = app.encoder_mut().update_texture::<R32_G32_B32_A32, (R32_G32_B32_A32, Float) >(
            &tex,
            None,
            gfx::texture::NewImageInfo {
                xoffset: 0,
                yoffset: 0,
                zoffset: 0,
                width: tex_w,
                height: tex_h,
                depth: 0,
                format: (),
                mipmap: 0,
            },
            cast_slice(img.as_slice()),
      
        ).unwrap();
        

        app.draw_fullscreen_quad(tex_view.clone());
        app.post_frame();
    };
}

pub fn cast_slice<A: Copy, B: Copy>(slice: &[A]) -> &[B] {
    use std::slice;
    let raw_len = mem::size_of::<A>().wrapping_mul(slice.len());
    let len = raw_len / mem::size_of::<B>();
    assert_eq!(raw_len, mem::size_of::<B>().wrapping_mul(len));
    unsafe {
        slice::from_raw_parts(slice.as_ptr() as *const B, len)
    }
}