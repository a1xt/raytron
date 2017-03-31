use {Bsdf};
use math::{self, Vector3f, Point3f, Ray3f, Coord, Dot};
use color::{self, Color};
use std::f32::consts::PI;

pub struct Diffuse {
    color: Color,
    emittance: Option<Color>,
}

impl Diffuse {
    pub fn new(color: Color, emittance: Option<Color>) -> Diffuse {
        Diffuse {
            color: color,
            emittance: emittance,
        }
    }
}

impl Bsdf for Diffuse {

    fn emittance(&self) -> Option<Color> {
        self.emittance
    }

    // fn reflectance(&self, _: &Vector3f, _: &Vector3f, _: &Vector3f) -> Color {
    //     self.color
    // }

    // fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f {
    //     Ray3f {
    //         origin: *surface_point,
    //         //dir: math::hs_uniform_sampling(surface_normal)
    //         dir: math::hs_cosine_sampling(surface_normal)
    //     }
    // }

    // fn brdf (&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color) {
    //     (
    //         Ray3f {
    //             origin: *surface_point,
    //             dir: math::hs_cosine_sampling(surface_normal),
    //         },
    //         self.color,
    //     )
    // }

    // fn reflectance(
    //     &self, 
    //     surface_normal: &Vector3f, 
    //     out_dir: &Vector3f,
    //     in_dir: &Vector3f
    // ) -> Color {

    //     color::mul_s(&self.color, 1.0 / PI)           
    // }

    fn sample(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f
    ) -> (Vector3f, Color, Coord) {

        let out_dir = math::hs_cosine_sampling(surface_normal);
        let cos_theta = surface_normal.dot(&out_dir);
        (out_dir, color::mul_s(&self.color, cos_theta as f32), 1.0)
    }

    fn sample_proj(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f
    ) -> (Vector3f, Color, Coord)
    {
        let out_dir = math::hs_cosine_sampling(surface_normal);
        let cos_theta = surface_normal.dot(&out_dir);
        (out_dir, self.color, 1.0)
    }

    fn eval(
        &self, 
        surface_normal: &Vector3f,
        in_dir: &Vector3f,
        out_dir: &Vector3f,        
    ) -> (Color, Coord)
    {
        let cos_theta = surface_normal.dot(&(-out_dir));
        let reflectance = color::mul_s(&self.color, 1.0 / PI);
        let pdf = cos_theta / PI as Coord;

        (reflectance, pdf)
    }

    fn eval_proj(
        &self, 
        surface_normal: &Vector3f,
        in_dir: &Vector3f, 
        out_dir: &Vector3f,        
    ) -> (Color, Coord)
    {
        let reflectance = color::mul_s(&self.color, 1.0 / PI);
        let pdf = 1.0 / PI as Coord;

        (reflectance, pdf)
    }

    // fn pdf(
    //     &self,
    //     surface_normal: &Vector3f, 
    //     in_dir: &Vector3f, 
    //     out_dir: &Vector3f
    // ) -> Coord {

    //     let cos_theta = surface_normal.dot(&(-in_dir));
    //     cos_theta / PI as Coord
    // }

    // /// pdf = pdf_proj * cos_theta
    // fn pdf_proj(
    //     &self,
    //     surface_normal: &Vector3f, 
    //     in_dir: &Vector3f, 
    //     out_dir: &Vector3f
    // ) -> Coord {

    //     1.0 / PI as Coord
    // }
}
