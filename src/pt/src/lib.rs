#![feature(step_by)]
#![feature(conservative_impl_trait)]

pub extern crate image;
pub extern crate rand;
pub extern crate scoped_threadpool;

pub mod math;
pub mod traits;
pub mod utils;
pub mod sphere;
//pub mod polygon;
pub mod sceneholder;
pub mod material;
pub mod renderer;
pub mod color;


pub use self::traits::{
    Bsdf,
    Surface,
    SceneHolder,
    Renderer,
    RenderCamera,
};

pub use self::sphere::{Sphere};
pub use self::sceneholder::{ShapeList};

pub use self::color::{Color, Image};



use self::math::{Point3f, Vector3f, Vector3, Vector4};

pub struct SurfacePoint<'a> {
    pub position: Point3f,
    pub normal: Vector3f,
    //pub bsdf: &'a Bsdf,
    pub bsdf: Box<Bsdf + 'a>,
    pub surface: &'a Surface,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSettings {
    samples_per_pixel: u32,
    path_depth: u32,
    fog_density: f32,

    render_block: (u32, u32),
    threads_num: u32,
}

impl RenderSettings {
    pub fn new (samples_per_pixel: u32, path_max_depth: u32) -> RenderSettings {
        RenderSettings {
            samples_per_pixel: samples_per_pixel,
            path_depth: path_max_depth,
            fog_density: 0.0,

            render_block: (1, 1),
            threads_num: 1,
        }
    }

    pub fn with_threads(&mut self, threads_num: u32, block_size: (u32, u32)) -> RenderSettings {
        self.threads_num = threads_num;
        self.render_block = block_size;

        *self
    }
}
