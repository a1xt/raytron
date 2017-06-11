#![feature(slice_patterns)]
#![feature(box_syntax)]
#![feature(const_fn)]
#![feature(associated_consts)]
#![feature(fixed_size_array)]
#![feature(conservative_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(specialization)]

extern crate core;
extern crate scoped_threadpool;
extern crate rand;
pub extern crate num;



pub mod math;
pub mod traits;
pub mod utils;
pub mod sphere;
pub mod polygon;
pub mod sceneholder;
pub mod bsdf;
pub mod renderer;
pub mod color;
pub mod aabb;
pub mod mesh;
pub mod texture;


pub use self::traits::{
    Bsdf,
    Surface,
    SceneHolder,
    Renderer,
    RenderCamera,
};

pub use self::sphere::{Sphere};
pub use self::polygon::{Polygon, PolygonS, PolygonR};
pub use self::polygon::material;
pub use self::polygon::vertex;
pub use self::mesh::Mesh;
pub use self::sceneholder::{ShapeList};
pub use self::color::{Color, Image};
pub use self::bsdf::BsdfRef;
pub use self::texture::{Tex, Texture};

use self::math::{Point3f, Vector3f};


pub struct SurfacePoint<'a> {
    pub position: Point3f,
    pub normal: Vector3f,
    pub bsdf: BsdfRef<'a>,
    pub surface: &'a Surface,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSettings {
    samples_per_pixel: u32,
    path_depth: u32,
    fog_density: f32,

    render_chunk: (u32, u32),
    threads_num: u32,
}

impl RenderSettings {
    pub fn new (samples_per_pixel: u32, path_max_depth: u32) -> RenderSettings {
        RenderSettings {
            samples_per_pixel: samples_per_pixel,
            path_depth: path_max_depth,
            fog_density: 0.0,

            render_chunk: (1, 1),
            threads_num: 1,
        }
    }

    pub fn with_threads(&mut self, threads_num: u32, chunk_size: (u32, u32)) -> RenderSettings {
        self.threads_num = threads_num;
        self.render_chunk = chunk_size;

        *self
    }
}
