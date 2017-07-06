#![feature(conservative_impl_trait)]
#![feature(box_syntax)]

#[macro_use] extern crate gfx;
pub extern crate gfx_window_glutin;
pub extern crate gfx_device_gl;
pub extern crate glutin;
pub extern crate nalgebra as na;
pub extern crate pathtracer as pt;

pub use pt::utils;

pub mod fpscamera;
pub mod camera_controller;
pub mod scenes;

use std::string::String;
use gfx::{Device, Factory};

use gfx::memory::Usage;
use gfx::format::{Rgba8, DepthStencil, R32_G32_B32_A32, Float};
use gfx::handle::{RenderTargetView, DepthStencilView, ShaderResourceView};
use gfx::Bundle;
use glutin::{CursorState, Event, GlRequest, Api};

use pt::math::{Real};

pub use camera_controller::{CameraController, FPSCameraController};
pub use fpscamera::{FPSCamera};

pub type ColorFormat = Rgba8;
pub type DepthFormat = DepthStencil;




const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub struct App<D: Device, F: Factory<D::Resources>> {
    encoder: gfx::Encoder<D::Resources, D::CommandBuffer>,
    device: D,
    factory: F,
    window: glutin::Window,
    events_loop: glutin::EventsLoop,
    color_out: RenderTargetView<<D as Device>::Resources, ColorFormat>,
    depth_out: DepthStencilView<<D as Device>::Resources, DepthFormat>,

    events: Vec<glutin::WindowEvent>,
    cam_ctrl: FPSCameraController,
    fsquad: Bundle<D::Resources, pipe::Data<D::Resources>>
}

impl<D: Device, F: Factory<D::Resources>> App<D, F> {
    pub fn new(screen_width: u32,
           screen_height: u32,
           title: String)
           -> App<gfx_device_gl::Device, gfx_device_gl::Factory> {

        use gfx::traits::FactoryExt;

        let gl_version = GlRequest::Specific(Api::OpenGl, (2, 1));
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(screen_width, screen_height)
            .with_gl(gl_version);

        let events_loop = glutin::EventsLoop::new();
        let (window, device, mut factory, rt_view, depth_view) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/default_120.glslv"),
            include_bytes!("shaders/default_120.glslf"),
            pipe::new()
        ).unwrap();

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&VBUF_DATA, ());

        // //let texels = [[0x20, 0xA0, 0xC0]];
        // let texels = [[0.0f32, 0.0, 1.0]];
        // let (_, texture_view) = factory.create_texture_const::<(gfx::format::R32_G32_B32_A32, gfx::format::Float)>(
        //     gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single), &texels[..]
        //     ).unwrap();

        let tex = factory.create_texture::<gfx::format::R32_G32_B32_A32> (
            gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single),
            1,
            gfx::SHADER_RESOURCE,
            Usage::Dynamic,
            Some(gfx::format::ChannelType::Float)
        ).unwrap();

        let texture_view = factory.view_texture_as_shader_resource::<(R32_G32_B32_A32, Float)>(
            &tex,
            (0,0),
            gfx::format::Swizzle::new()
        ).unwrap();

        let sinfo = gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Bilinear,
            gfx::texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbuf: vbuf,
            tex: (texture_view, factory.create_sampler(sinfo)),
            out_color: rt_view.clone(),
            out_depth: depth_view.clone(),
        };

        App {
            encoder: encoder,
            events_loop,
            window: window,
            device: device,
            factory: factory,
            color_out: rt_view,
            depth_out: depth_view,
            events: Vec::new(),
            cam_ctrl: FPSCameraController::new(
                FPSCamera::new(
                    screen_width,
                    screen_height,
                    (90.0 as Real).to_radians(),
                    1.0,
                    10_000.0),
                0.5,
                1.0),
            fsquad: Bundle::new(slice, pso, data),
        }

    }

    pub fn run(&mut self) -> bool {
        let mut quite: bool = false;

        self.events.clear();
        {
            let events = &mut self.events;
            self.events_loop.poll_events(|event| {
                let Event::WindowEvent{event: e, ..} = event;
                events.push(e);
            });
        }

        use glutin::{WindowEvent, VirtualKeyCode};
        for event in self.events.iter() {
            match *event {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                WindowEvent::Closed => {
                    quite = true;
                    break;
                },
                _ => (),
            }
        }

        let ref events = self.events;
        let ref window = self.window;
        let ref mut cam_ctrl = self.cam_ctrl;

        cam_ctrl.on_frame(
            events.iter(),
            &mut |x: i32, y: i32| {
                window.set_cursor_position(x, y).unwrap_or(());
            },
            &mut |lock: bool| {
                let state = if lock {
                    CursorState::Grab
                } else {
                    CursorState::Normal
                };
                window.set_cursor_state(state).unwrap_or(());
            }
        );

        !quite
    }

    pub fn clear_frame(&mut self) {
        self.encoder.clear(&self.color_out, CLEAR_COLOR);
        self.encoder.clear_depth(&self.depth_out, 1.0);
    }


    pub fn post_frame(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }

    pub fn draw_fullscreen_quad(&mut self, tex_view: ShaderResourceView<D::Resources, [f32; 4]>) {
        let sinfo = gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Bilinear,
            gfx::texture::WrapMode::Clamp);

        self.fsquad.data.tex = (tex_view, self.factory.create_sampler(sinfo));
        self.fsquad.encode(&mut self.encoder);
    }

    pub fn encoder_mut(&mut self) -> &mut gfx::Encoder<D::Resources, D::CommandBuffer> {
        &mut self.encoder
    }

    pub fn encoder(&self) -> &gfx::Encoder<D::Resources, D::CommandBuffer> {
        &self.encoder
    }

    pub fn device(&self) -> &D {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn factory_mut(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    pub fn window(&self) -> &glutin::Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut glutin::Window {
        &mut self.window
    }

    pub fn events(&self) -> &[glutin::WindowEvent] {
        self.events.as_slice()
    }

    pub fn cam_ctrl_mut(&mut self) -> &mut FPSCameraController {
        &mut self.cam_ctrl
    }

    pub fn cam_ctrl(&mut self) -> &FPSCameraController {
        &self.cam_ctrl
    }
}

impl App<gfx_device_gl::Device, gfx_device_gl::Factory> {
    pub fn resize(&mut self, width: u32, height: u32, scale: f32) {
        let w = (width as f32 * scale) as u32;
        let h = (height as f32 * scale) as u32;        
        self.window.set_inner_size(w, h);
        let (color, depth) = gfx_window_glutin::new_views(&self.window);
        self.color_out = color.clone();
        self.depth_out = depth.clone();
        self.fsquad.data.out_color = color;
        self.fsquad.data.out_depth = depth;
        println!("win resized: w = {}, h = {}", width, height);
    }
}

const VBUF_DATA: [Vertex; 6] = [
    Vertex { pos: [-1.0, -1.0, 0.0], tex_coord: [0.0, 1.0]},
    Vertex { pos: [-1.0,  1.0, 0.0], tex_coord: [0.0, 0.0]},
    Vertex { pos: [ 1.0,  1.0, 0.0], tex_coord: [1.0, 0.0]},

    Vertex { pos: [-1.0, -1.0, 0.0], tex_coord: [0.0, 1.0]},
    Vertex { pos: [ 1.0,  1.0, 0.0], tex_coord: [1.0, 0.0]},
    Vertex { pos: [ 1.0, -1.0, 0.0], tex_coord: [1.0, 1.0]},
];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "pos3",
        tex_coord: [f32; 2] = "tex_coord",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "tex2",
        out_color: gfx::RenderTarget<::ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}