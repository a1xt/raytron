use math::{self, Norm, Point3f, Vector3f, Ray3f, Real};
//use math::{Dot};
use super::{Surface, SurfacePoint, Bsdf};
use std::boxed::Box;
//use std::f32::consts::PI;
use color::{self, Color};

#[derive(Clone, Copy, Debug)]
pub struct Sphere<B: Bsdf + Clone> {
    pub position: Point3f,
    pub radius: Real,
    //bsdf: Box<Bsdf>,
    pub bsdf: B,
}

impl<B: Bsdf + Clone> Sphere<B> {
    pub fn new(position: Point3f, radius: Real, mat: B) -> Sphere<B> {
        Sphere {
            position: position,
            radius: radius,
            bsdf: mat,
        }
    }

    pub fn bsdf<'s>(&'s self) -> Box<Bsdf + 's> {
        Box::new(self.bsdf.clone())
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
        //(self.position - *point).normalize()
    }
}

impl<B: Bsdf + Clone> Surface for Sphere<B> {

    fn intersection (&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some(t) = math::intersection_with_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    bsdf: self.bsdf(),
                    surface: self,
                }
            ))
        } else {
            None
        }
    }

    fn area (&self) -> Real {
        use std::f32::consts::PI;
        4.0 * (PI as Real) * self.radius * self.radius
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

    fn sample_surface(&self, _: &Point3f) -> (SurfacePoint, Real) {
        // let view_dir = (*view_point - self.position).normalize();
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

    fn pdf(&self, _: &Point3f, _ : &Point3f) -> Real {
        //2.0 / self.area()        
        1.0 / self.area()
    }

}