use math::{Ray3f, Matrix4f, Vector3f, Point3f, Coord, Dot};
use math::{Norm};
use math;
use super::{SurfacePoint, Color, RenderSettings, Image};
use std::f32;
use image::ImageBuffer;
use color;
use rand::{Closed01};
use rand;

pub use renderer::Renderer;
pub use sceneholder::SceneHolder;

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

    /// ∫ Le dA
    fn total_emittance(&self) -> Option<Color>;

    fn area (&self) -> Coord;
    fn normal_at(&self, pos: &Point3f) -> Vector3f;

    // return (random point, pdf)
    fn sample_surface(&self, view_point: &Point3f) -> (SurfacePoint, Coord);
    fn pdf(&self,  point_at_surface: &Point3f, view_point: &Point3f) -> Coord;
    // fn sample_surface_p(&self, view_point: &Point3f) -> (SurfacePoint, Coord);
    // fn sample_surface_d(&self, view_point: &Point3f) -> (SurfacePoint Coord);

    // fn pdf_p(&self,  point_at_surface: &Point3f, view_point: &Point3f) -> Coord;
    // fn pdf_d(&self,  point_at_surface: &Point3f, view_point: &Point3f) -> Coord;
}

pub trait Bsdf : Sync {

    fn emittance(&self) -> Option<Color>;
    //fn reflectance(&self, ray: &Vector3f, reflected_ray: &Vector3f, normal: &Vector3f) -> Color;
    //fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f;

    /// return (reflected ray, reflectance)
    //fn brdf(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color);

    fn eval(
        &self, 
        surface_normal: &Vector3f,
        in_dir: &Vector3f,
        out_dir: &Vector3f,        
    ) -> (Color, Coord);

    fn sample(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f
    ) -> (Vector3f, Color, Coord);

    fn eval_proj(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f,
        out_dir: &Vector3f,
    ) -> (Color, Coord) {
        let (fr, pdf) = self.eval(surface_normal, in_dir, out_dir);
        let cos_theta = surface_normal.dot(&out_dir);
        (fr, pdf / cos_theta)
    }  

    fn sample_proj(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f
    ) -> (Vector3f, Color, Coord) {
        let (ray, fr, pdf) = self.sample(surface_normal, in_dir);
        let cos_theta = surface_normal.dot(&ray);

        (ray, fr, pdf / cos_theta)
    }

}