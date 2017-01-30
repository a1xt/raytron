use math::{intersection_with_sphere, BaseFloat, Norm, Point3f, Vector3f, Ray3f};
use super::{Surface, SurfacePoint, Material};
use std::boxed::Box;
use math;

//#[derive(Clone)]
pub struct Sphere {
    pub position: Point3f,
    pub radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(position: Point3f, radius: f32, mat: Box<Material>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            material: mat,
        }
    }

    pub fn material(&self) -> &Material {
        self.material.as_ref()
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
        //(self.position - *point).normalize()
    }
}

impl Surface for Sphere {

    fn intersection (&self, ray: &Ray3f) -> Option<(f32, SurfacePoint)> {
        if let Some(t) = intersection_with_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    material: self.material(),
                }
            ))
        } else {
            None
        }
    }

    fn random_point (&self) -> SurfacePoint {
        let norm = math::sph_uniform_sampling();
        let pos: Point3f = self.position +  (norm * self.radius);
        
        SurfacePoint {
            position: pos,
            //position: self.position,
            normal: norm,
            material: self.material(),
        }
    }

}