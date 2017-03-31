use math::{self, intersection_with_sphere, BaseFloat, Norm, Point3f, Vector3f, Ray3f, Coord, Dot};
use super::{Surface, SurfacePoint, Bsdf};
use std::boxed::Box;
use std::f32::consts::PI;
use utils::consts;
use color::{self, Color};
use rand;

//#[derive(Clone)]
pub struct Sphere {
    pub position: Point3f,
    pub radius: Coord,
    bsdf: Box<Bsdf>,
}

impl Sphere {
    pub fn new(position: Point3f, radius: Coord, mat: Box<Bsdf>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            bsdf: mat,
        }
    }

    pub fn bsdf(&self) -> &Bsdf {
        self.bsdf.as_ref()
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
        //(self.position - *point).normalize()
    }
}

impl Surface for Sphere {

    fn intersection (&self, ray: &Ray3f) -> Option<(Coord, SurfacePoint)> {
        if let Some(t) = intersection_with_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos + norm * consts::POSITION_EPSILON,
                    normal: norm,
                    bsdf: self.bsdf(),
                    surface: self,
                }
            ))
        } else {
            None
        }
    }

    // fn random_point (&self) -> SurfacePoint {
    //     let norm = math::sph_uniform_sampling();
    //     let pos: Point3f = self.position +  (norm * self.radius);
        
    //     SurfacePoint {
    //         position: pos,
    //         //position: self.position,
    //         normal: norm,
    //         bsdf: self.bsdf(),
    //     }
    // }

    fn area (&self) -> Coord {
        use std::f32::consts::PI;
        4.0 * (PI as Coord) * self.radius * self.radius
    }

    fn total_emittance(&self) -> Option<Color> {
        if let Some(e) = self.bsdf.emittance() {
            Some(color::mul_s(&e, self.area() as f32))
        } else {
            None
        }
    }

    fn normal_at(&self, pos: &Point3f) -> Vector3f {
        self.normal_to(pos)
    }

    fn sample_surface(&self, view_point: &Point3f) -> (SurfacePoint, Coord) {
        let view_dir = (*view_point - self.position).normalize();
        // let normal = math::hs_uniform_sampling(&view_dir);
        // let pdf = 2.0 / self.area();
        let normal = math::sph_uniform_sampling();
        let pdf = 1.0 / self.area();
        let pos = self.position + (normal * self.radius);

        (SurfacePoint {
            position: pos,
            normal: normal,
            bsdf: self.bsdf(),
            surface: self,
        },
        pdf)
    }

    fn pdf(&self, view_point: &Point3f, point_at_surface: &Point3f) -> Coord {
        //2.0 / self.area()        
        1.0 / self.area()
    }

}