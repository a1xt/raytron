pub extern crate pt_app;
pub extern crate gfx;
pub extern crate gfx_device_gl as gfx_gl;
pub extern crate glutin;
pub extern crate time;
pub extern crate image;
pub extern crate rand;
pub extern crate tobj;

pub use pt_app::*;
pub use pt_app::camera_controller::{CameraController, FPSCameraController};

use pt_app::{App};
use pt_app::pt::renderer::{PathTracer, DbgRayCaster};
use pt_app::pt::{Image, RenderSettings, TexView, Texture, Mesh};
use pt_app::pt::color::{self, Color, Rgb, Luma};
use pt_app::pt::traits::{Renderer, SceneHandler, Vertex, Material};
use pt_app::pt::material::{PbrTex};
use pt_app::pt::vertex::TbnVertex;
use pt_app::pt::math::{Real, Norm, Cross, Vector3f, Point3f, Vector2};
use pt_app::pt::math;
use pt_app::pt::utils::consts;
use image::hdr;

use std;
use std::mem;
use std::sync::Arc;
use std::path::Path;
use std::io::Write;
use std::string::{String, ToString};
use gfx::format::{Formatted, ChannelTyped, Rgba32F};
use gfx::{Factory, Device};

use glutin::{WindowEvent, ElementState, VirtualKeyCode};

type GLFactory = gfx_gl::Factory;
type GLDevice = gfx_gl::Device;
type TexFormat = Rgba32F;

pub struct ExampleApp {
    app: App<GLDevice, GLFactory>,
    screen_width: usize,
    screen_height: usize,
    dbg_rdr: DbgRayCaster,
    dbg_setup: RenderSettings,
    //img: Image,
    tex: gfx::handle::Texture<<GLDevice as Device>::Resources, <TexFormat as Formatted>::Surface>,
    tex_view: gfx::handle::ShaderResourceView<<GLDevice as Device>::Resources, <TexFormat as Formatted>::View>,
}

impl ExampleApp {

    pub fn launch<T: AppState >(state: &mut T) {
        //let mut state = T::new();
        let mut ex_app = {
            let builder = state.init();
            builder.build()
        };

        let mut dbg = true;
        let mut pass_num = 0;
        //let mut last_time = 0;
        let mut total_time = 0u64;
        let mut run = true;
        let mut img = Image::new(ex_app.screen_width, ex_app.screen_height);

        while run {
            {
                let scene = state.create_scene();
                let (mut rdr, rdr_setup) = state.create_renderer();
                state.init_camera(ex_app.app.cam_ctrl_mut());
            
                while !state.need_update() && run {        
                    {
                        let ExampleApp { 
                            ref mut app,
                            ref mut dbg_rdr,
                            dbg_setup,
                            ..
                        } = ex_app;

                        if dbg {
                            dbg_rdr.render_scene_threaded(scene.as_ref(), app.cam_ctrl().camera(), &dbg_setup, &mut img);
                        } else {
                            let start_time = time::precise_time_ns();
                            if pass_num == 0 {
                                rdr.pre_render(scene.as_ref(), app.cam_ctrl().camera(), &rdr_setup);
                                total_time = 0;
                            }
                            rdr.render_pass_threaded(scene.as_ref(), app.cam_ctrl().camera(), &rdr_setup, pass_num, &mut img);
                            pass_num += 1;
                            let frame_time = time::precise_time_ns() - start_time;
                            total_time = total_time + frame_time;
                            println!("pass_num: {}, frame time: {:.2}, average time: {:.2}", 
                                pass_num, (frame_time as f64) * 1.0e-9, (total_time as f64) / (pass_num as f64) * 1.0e-9);
                        }
                    }

                    let mut res_img = img.clone();
                    state.post_process(&mut res_img);

                    for event in ex_app.app.events().iter() {
                        match *event {
                            WindowEvent::KeyboardInput(el_state, _, Some(key), _) => {
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
                                        ex_app.save_img(&res_img, None);
                                    },

                                    _ => (),
                                }
                            }

                            _ => {}
                        }
                    }

                    // let app = &ex_app.app;
                    // app.clear_frame();

                    // let _ = app.encoder_mut().update_texture::<<TexFormat as Formatted>::Surface, TexFormat >(
                    //     tex,
                    //     None,
                    //     gfx::texture::NewImageInfo {
                    //         xoffset: 0,
                    //         yoffset: 0,
                    //         zoffset: 0,
                    //         width: img.width() as u16,
                    //         height: img.height() as u16,
                    //         depth: 0,
                    //         format: (),
                    //         mipmap: 0,
                    //     },
                    //     cast_slice(img.as_slice()),
                
                    // ).unwrap();
                    

                    // app.draw_fullscreen_quad(tex_view.clone());
                    // app.post_frame();
                    ex_app.draw_tex(&res_img);
                    run = ex_app.app.run();
                };
            }
            state.update();
        }
    }

    fn resize(&mut self, (w, h): (usize, usize)) {
        let aspect = w as f32 / h as f32;
        let (tex_w, tex_h) = (w as u16, h as u16);
        let (scr_w, scr_h) = {
            let mut w1 = self.screen_width;
            let mut h1 = self.screen_height;
            if w != self.screen_width {
                w1= self.screen_width;
                h1 = (w1 as f32 / aspect) as usize;
            }
            let mut w2 = w1;
            let mut h2 = h1;
            if h1 > self.screen_height {
                h2 = self.screen_height;
                w2 = (h2 as f32 * aspect) as usize;
            }
            (w2 as u32, h2 as u32)
        } ;
        let tex = self.app.factory_mut().create_texture::<<TexFormat as Formatted>::Surface> (
            gfx::texture::Kind::D2(tex_w, tex_h, gfx::texture::AaMode::Single),
            1,
            gfx::SHADER_RESOURCE,
            gfx::memory::Usage::Dynamic,
            Some(<<TexFormat as Formatted>::Channel as ChannelTyped>::get_channel_type()))
            .unwrap();

        let tex_view = self.app.factory_mut().view_texture_as_shader_resource::<TexFormat>(
            &tex,
            (0,0),
            gfx::format::Swizzle::new())
            .unwrap();

        self.tex = tex;
        self.tex_view = tex_view;
        self.app.resize(scr_w, scr_h, 1.0);
        // self.screen_width = w;
        // self.screen_height = h;
    }

    fn draw_tex(&mut self, img: &Image) {
        use gfx::texture::Kind;
        let (tex_w, tex_h) = {
            if let Kind::D2(w, h, _) = self.tex.get_info().kind {
                (w, h)
            } else {
                unreachable!()
            }
        };
        if tex_w != img.width() as u16 || tex_h != img.height() as u16 {
            self.resize((img.width(), img.height()));
        }
        self.app.clear_frame();
        let _ = self.app.encoder_mut().update_texture::<<TexFormat as Formatted>::Surface, TexFormat >(
            &self.tex,
            None,
            gfx::texture::NewImageInfo {
                xoffset: 0,
                yoffset: 0,
                zoffset: 0,
                width: img.width() as u16,
                height: img.height() as u16,
                depth: 0,
                format: (),
                mipmap: 0,
            },
            cast_slice(img.as_slice()),
    
        ).unwrap();
        self.app.draw_fullscreen_quad(self.tex_view.clone());
        self.app.post_frame();
    }

    fn save_img(&self, img: &Image, file_name: Option<String>) {
        let image_name = if let Some(name) = file_name {
            name
        } else {
            "res_img/".to_string() + rand::random::<u16>().to_string().as_ref() + ".png"
        };
        println!("img_name: {}", image_name);

        let mut buf: Vec<u8> = Vec::new();
        for c in img.pixels() {
            let cc: color::Rgb<u8> = c.into();
            buf.push(cc.r);
            buf.push(cc.g);
            buf.push(cc.b);
        }

        image::save_buffer(&std::path::Path::new(&image_name), 
                            buf.as_ref(), 
                            img.width() as u32, 
                            img.height() as u32, 
                            image::RGB(8)).unwrap();
        println!("image saved!");
    }

}


pub struct ExampleAppBuilder {
    name: Option<String>,
    size: Option<(usize, usize)>,
    dbg_setup: Option<RenderSettings>,
}

impl<'a> ExampleAppBuilder {
    pub fn new() -> ExampleAppBuilder {
        ExampleAppBuilder{
            name: None,
            size: None,
            dbg_setup: None,
        }
    }

    pub fn name(self, name: String) -> ExampleAppBuilder {
        ExampleAppBuilder {
            name: Some(name),
            .. self
        }
    }

    pub fn size(self, width: usize, height: usize) -> ExampleAppBuilder {
        ExampleAppBuilder {
            size: Some((width, height)),
            .. self
        }
    }

    pub fn dbg_rdr_setup(self, dbg_setup: RenderSettings) -> ExampleAppBuilder {
        ExampleAppBuilder {
            dbg_setup: Some(dbg_setup),
            .. self
        }
    }

    pub fn build(self) -> ExampleApp {
        let name = self.name.unwrap_or("Example".to_string());
        let (screen_width, screen_height) = self.size.unwrap_or((800, 600));
        let mut app = App::<GLDevice, GLFactory>::new(screen_width as u32, screen_height as u32, name);

        
        let dbg_render_chunk = (100, 75);            
        let dbg_setup_d = RenderSettings::new(1, 1).with_threads(4, dbg_render_chunk);
        let dbg_setup = self.dbg_setup.unwrap_or(dbg_setup_d);

        let (tex_w, tex_h) = (screen_width as u16, screen_height as u16);
        let tex = app.factory_mut().create_texture::<<TexFormat as Formatted>::Surface> (
            gfx::texture::Kind::D2(tex_w, tex_h, gfx::texture::AaMode::Single),
            1,
            gfx::SHADER_RESOURCE,
            gfx::memory::Usage::Dynamic,
            Some(<<TexFormat as Formatted>::Channel as ChannelTyped>::get_channel_type()))
            .unwrap();

        let tex_view = app.factory_mut().view_texture_as_shader_resource::<TexFormat>(
            &tex,
            (0,0),
            gfx::format::Swizzle::new())
            .unwrap();

        ExampleApp {
            app,
            screen_width,
            screen_height,
            dbg_rdr: DbgRayCaster::new(),
            dbg_setup,
            //img: Image::new(screen_width, screen_height),
            tex,
            tex_view,
        }
    }
}

pub trait AppState {
    fn new() -> Self where Self: Sized;
    fn init(&mut self) -> ExampleAppBuilder;
    fn update(&mut self) { }
    fn need_update(&self) -> bool { false }
    fn post_process(&self, &mut Image) { }
    fn init_camera(&self, &mut FPSCameraController) { }
    //fn update_camera(&self, &mut CameraController) { }

    fn create_renderer<'s>(&'s self) -> (Box<Renderer<SceneHandler + 's> + 's>, RenderSettings) {
        let pt_render_chunk = (80, 60);
        let rdr_setup = RenderSettings::new(128, 6).with_threads(4, pt_render_chunk);       
        let rdr = box PathTracer::new(&rdr_setup).with_direct_illumination(0.75, 0.25);
        (rdr, rdr_setup)
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHandler + 's>;
}

pub fn tone_mapping_exp<'a, T: TexView<Color>>(img: &'a mut T, t: f32) {
    for j in 0..img.height() {
        for i in 0..img.width() {
            let c = img.pixel(i, j);
            let t = 1.125;
            let x = Rgb::new(
                1.0 - ((-t) * c.r).exp(),
                1.0 - ((-t) * c.g).exp(),
                1.0 - ((-t) * c.b).exp(),
            );
            img.set_pixel(i, j, x);
        }
    }
}

pub fn tone_mapping_simple<'a, T: TexView<Color> + 'a>(img: &'a mut T) {
    for j in 0..img.height() {
        for i in 0..img.width() {
            let c = img.pixel(i, j);
            let t = 1.125;
            let x = Rgb::new(
                c.r / (1.0 + c.r),
                c.g / (1.0 + c.g),
                c.b / (1.0 + c.b),
            );
            img.set_pixel(i, j, x);
        }
    }    
}

pub fn gamma_encoding<'a, T: TexView<Color> + 'a>(img: &'a mut T) {
    let g = 1.0 / 2.2;
    for j in 0..img.height() {
        for i in 0..img.width() {
            let mut c = img.pixel(i, j);
            c.r = c.r.powf(g);
            c.g = c.g.powf(g);
            c.b = c.b.powf(g);
            img.set_pixel(i, j, c);
        }
    }
}

pub fn gamma_decoding<'a, T: TexView<Color> + 'a>(img: &'a mut T) {
    let g = 2.2;
    for j in 0..img.height() {
        for i in 0..img.width() {
            let mut c = img.pixel(i, j);
            c.r = c.r.powf(g);
            c.g = c.g.powf(g);
            c.b = c.b.powf(g);
            img.set_pixel(i, j, c);
        }
    }
}

macro_rules! mono_texture {
    ($color:expr) => {{
        use pt_app::pt::texture::Texture;
        let mut tex = Texture::new(1, 1);
        tex.set_pixel(0, 0, $color);
        tex
    }};
}

pub fn load_hdr<T: TexView<Rgb>>(path: String) -> T {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

    print!("loading hdr image ...");
    std::io::stdout().flush();

    let img_file = File::open(path).unwrap();
    let hdrdecoder = hdr::HDRDecoder::new(BufReader::new(img_file)).unwrap();
    let hdr_meta = hdrdecoder.metadata();
    let mut img = T::new(hdr_meta.width as usize, hdr_meta.height as usize);
    let hdr_data = hdrdecoder.read_image_hdr().unwrap();
    let h = img.height();
    for j in 0..(hdr_meta.height as usize) {
        for i in 0..(hdr_meta.width as usize) {
            let p = hdr_data[j * (hdr_meta.width as usize) + i];
            let c = Rgb::new(p[0], p[1], p[2]);
            img.set_pixel(i, h - j - 1, c.into());
        }
    }
    println!("done! (width: {}, height: {}", hdr_meta.width, hdr_meta.height);
    img        
}

pub fn load_texture<C, T: TexView<C>, F>(path: String, map_color: F) -> T
    where C: Into<Rgb<Real>> + Into<Color>,
          F: Fn(image::Rgba<u8>) -> C,
{
    use image::GenericImage;
    use std::path::Path;

    print!("loading texture: `{}` ... ", &path);
    let _ = std::io::stdout().flush();

    let img = image::open(&Path::new(&path)).unwrap();
    let mut tex = T::new(img.width() as usize, img.height() as usize);
    let h = tex.height();
    for j in 0..img.height() {
        for i in 0..img.width() {
            let p = img.get_pixel(i, j);
            let c = map_color(p);
            tex.set_pixel(i as usize, h - (j as usize) - 1, c);
        }
    }
    println!("done!");
    tex
}

pub fn load_texture_rgb64<T: TexView<Rgb<Real>>>(path: String, srgb_encoded: bool) -> T {
    load_texture::<Rgb<f64>, _, _>(path, move |c| {
        let mut p = Rgb::<u8>::new(c[0], c[1], c[2]).into() :Rgb<f64>;
        if srgb_encoded {
            p.r = p.r.powf(2.2);
            p.g = p.g.powf(2.2);
            p.b = p.b.powf(2.2);
        }
        p
    })
}

pub fn load_texture_luma64<T: TexView<Luma<Real>>>(path: String, srgb_encoded: bool) -> T {
    load_texture::<Luma<f64>, _, _>(path, move |c| {
        let mut p = Luma::<u8>::new(c[0]).into() :Luma<f64>;
        if srgb_encoded {
            p.luma = p.luma.powf(2.2);
        }
        p
    })
}

pub fn load_pbr(path: String) -> Arc<PbrTex<'static, Rgb<Real>, Luma<Real>>> {
    use std::sync::Arc;

    let basecolor_tex: Texture<Rgb<f64>> = load_texture_rgb64(path.clone() + "basecolor.png", true);
    let normal_tex: Texture<Rgb<f64>> = load_texture_rgb64(path.clone() + "normal.png", false);
    let roughness_tex: Texture<Luma<f64>> = load_texture_luma64(path.clone() + "roughness.png", false);
    let metal_tex: Texture<Luma<f64>> = load_texture_luma64(path.clone() + "metalness.png", false);
    let spec_tex: Texture<Luma<f64>> = mono_texture!(Luma::from(1.0));

    let pbrtex_mat: Arc<PbrTex< Rgb<Real>, Luma<Real> >> = Arc::new(PbrTex::new(
        basecolor_tex,
        normal_tex,
        roughness_tex,
        spec_tex,
        metal_tex));

    pbrtex_mat
}

pub fn load_obj_pbr<'a, M, F>(
    path: String,
    mut material: M,
    mut pos_transform: F)
    -> Vec<Mesh<'a, TbnVertex>>
    where M: FnMut(String) -> Arc<Material<TbnVertex> + 'a>,
          F: FnMut(Point3f) -> Point3f,
{
    println!("loading model: `{}` ... ", &path);
    let _ = std::io::stdout().flush();

    let (models, _) = tobj::load_obj(&Path::new(&path)).unwrap();
    let mut mvec = Vec::with_capacity(models.len());
    let mut total_faces = 0;
    let mut total_vertices = 0;

    for (i, tobj::Model{ mesh: model_mesh, name: mesh_name}) in models.into_iter().enumerate() {
        let mut vnum = 0;
        let mut fnum = 0;
        let mut mesh = Mesh::new();
        let mat = material(mesh_name.clone());
        if !model_mesh.texcoords.is_empty() {
            let mut vertices = Vec::with_capacity(model_mesh.indices.len() / 3);

            for (pos, uv) in model_mesh.positions.chunks(3).zip(model_mesh.texcoords.chunks(2)) {
                let p = Point3f::new(pos[0] as Real, pos[1] as Real, pos[2] as Real);
                vertices.push(TbnVertex::new(
                    pos_transform(p),
                    math::zero(),
                    math::zero(),
                    math::zero(),
                    Vector2::new(uv[0], uv[1]),
                ));
            }

            for ix in model_mesh.indices.chunks(3) {
                let v0 = vertices[ix[0] as usize];
                let v1 = vertices[ix[2] as usize];
                let v2 = vertices[ix[1] as usize];
                let duv1 = v1.uv - v0.uv;
                let duv2 = v2.uv - v0.uv;
                let (t, b) = math::calc_tangent(
                    (&(v1.position - v0.position), duv1.x as Real, duv1.y as Real),
                    (&(v2.position - v0.position), duv2.x as Real, duv2.y as Real));
                let n = t.cross(&b).normalize();

                vertices[ix[0] as usize].tangent += t;
                vertices[ix[0] as usize].bitangent += b;
                vertices[ix[0] as usize].normal += n;

                vertices[ix[1] as usize].tangent += t;
                vertices[ix[1] as usize].bitangent += b;
                vertices[ix[1] as usize].normal += n;
                
                vertices[ix[2] as usize].tangent += t;
                vertices[ix[2] as usize].bitangent += b;
                vertices[ix[2] as usize].normal += n;
            }

            for mut v in vertices {
                v.tangent = v.tangent.normalize();
                v.bitangent = v.bitangent.normalize();
                v.normal = v.normal.normalize();
                mesh.add_vertex(v);
                vnum += 1;
            }

            for ix in model_mesh.indices.chunks(3) {
                mesh.add_face([ix[0], ix[2], ix[1]], mat.clone()).unwrap();
                fnum += 1;
            }
            
            println!(" -- mesh[{}] loaded - name: {}, vertices: {}, faces: {}", i, mesh_name, vnum, fnum);
            total_faces += fnum;
            total_vertices += vnum;
        } else {
            println!(" -- warning! [{}] mesh is skipped because texture coodinates are not found", mesh_name);
        }
        mvec.push(mesh);
    }
    println!("done! (total vertices: {}, total faces: {})", total_vertices, total_faces);
    mvec
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