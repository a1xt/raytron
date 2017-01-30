extern crate pt_app;
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate glutin;


use gfx::Factory;

//use pt_app::camera_controller::{CameraController, FPSCameraController};
//use pt_app::utils::camera::{FPSCamera, RenderCamera};
use pt_app::{App};
use glutin::CursorState;
use pt_app::utils;

fn main () {
    let width = 800u32;
    let height = 600u32;

    let mut app = App::<gfx_gl::Device, gfx_gl::Factory>::new(width, height, "apptest".to_string());

    let texels = [[0xA0, 0x20, 0xC0, 0x00]];
    let (_, texture_view) = app.factory_mut().create_texture_const::<gfx::format::Rgba8>(
        gfx::tex::Kind::D2(1, 1, gfx::tex::AaMode::Single), &[&texels]
        ).unwrap();


    while app.run() {
        app.clear_frame();
        app.draw_fullscreen_quad(texture_view.clone());
        app.post_frame();
    }
}