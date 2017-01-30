pub extern crate image;
pub extern crate rand;

pub mod math;
pub mod traits;
pub mod utils;
pub mod sphere;
pub mod sceneholder;
pub mod material;
pub mod renderer;
pub mod color;


pub use self::traits::{
    Material,
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
    pub material: &'a Material,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSettings {
    samples_per_pixel: u32,
    path_depth: u32,
    fog_density: f32,
}

impl RenderSettings {
    pub fn new (samples_per_pixel: u32, path_max_depth: u32) -> RenderSettings {
        RenderSettings {
            samples_per_pixel: samples_per_pixel,
            path_depth: path_max_depth,
            fog_density: 0.0,
        }
    }
}