use math::{Ray3f, Matrix4f, Vector3f, Point3f, Coord};
use math::{Norm};
use math;
use super::{SurfacePoint, Color, RenderSettings, Image};
use std::f32;
use image::ImageBuffer;
use color;
use rand::{Closed01};
use rand;

pub use renderer::Renderer;

pub trait RenderCamera {
    fn view_matrix(&self) -> Matrix4f;
    fn proj_matrix(&self) -> Matrix4f;

    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn aspect(&self) -> Coord;
    fn znear(&self) -> Coord;
    fn zfar(&self) -> Coord;
    fn fovx(&self) -> Coord;

    fn pos(&self) -> Point3f;
    fn up_vec(&self) -> Vector3f;
    fn forward_vec(&self) -> Vector3f;
    fn right_vec(&self) -> Vector3f;
}

pub trait Surface : Sync {
    /// return (t, sp)
    fn intersection (&self, ray: &Ray3f) -> Option<(Coord, SurfacePoint)>;
    fn random_point (&self) -> SurfacePoint;
}

pub trait SceneHolder {
    fn intersection_with_scene(&self, ray: &Ray3f) -> Option<SurfacePoint>;
    fn random_light_source<'s>(&'s self) -> Option<&'s Surface>;
    //fn ligth_sources();
}


pub trait Material : Sync {

    // fn emission(&self) -> Option<f32>;
    // fn color(&self) -> Color;
    // fn reflectance(&self) -> f32;
    // fn reflect_ray<F: BaseFloat>(&self, ray: Ray<F>, normal: Vector3<F>);

    fn emission(&self) -> Option<Color>;
    fn reflectance(&self, ray: &Vector3f, reflected_ray: &Vector3f, normal: &Vector3f) -> Color;
    fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f;

    /// return (reflected ray, reflectance)
    fn brdf(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color);
    

}