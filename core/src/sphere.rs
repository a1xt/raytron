use {Bsdf, Surface, SurfacePoint};
use aabb::{Aabb3, HasBounds};
use bsdf::BsdfRef;
use color::Color;
use math::{self, Norm, Point3f, Ray3f, Real, Vector3f};
use std::borrow::{Borrow, BorrowMut};
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub position: Point3f,
    pub radius: Real,
    pub bsdf: Arc<Bsdf>,
}

impl Sphere {
    pub fn new(position: Point3f, radius: Real, mat: Arc<Bsdf>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            bsdf: mat,
        }
    }

    pub fn bsdf(&self) -> BsdfRef {
        BsdfRef::Ref(self.bsdf.as_ref())
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
    }
}

impl Surface for Sphere {
    #[inline]
    fn is_emitter(&self) -> bool {
        self.bsdf.radiance().is_some()
    }

    fn intersection(&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some(t) = math::intersection_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    bsdf: self.bsdf(),
                    surface: self,
                },
            ))
        } else {
            None
        }
    }

    #[inline]
    fn area(&self) -> Real {
        4.0 * (PI as Real) * self.radius * self.radius
    }

    #[inline]
    fn total_radiance(&self) -> Option<Color> {
        if let Some(e) = self.bsdf.radiance() {
            Some(e * self.area() as f32)
        } else {
            None
        }
    }

    #[inline]
    fn normal_at(&self, pos: &Point3f) -> Vector3f {
        self.normal_to(pos)
    }

    fn sample_surface_p(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let view_dir = (*view_point.0 - self.position).normalize();
        let normal = math::hs_uniform_sampling(&view_dir);
        // let normal = math::sph_uniform_sampling();
        let pdf = 2.0 / self.area();
        let pos = self.position + (normal * self.radius);

        (
            SurfacePoint {
                position: pos,
                normal: normal,
                bsdf: self.bsdf(),
                surface: self,
            },
            pdf,
        )
    }

    #[inline]
    fn pdf_p(&self, _: (&Point3f, &Vector3f), _: (&Point3f, &Vector3f)) -> Real {
        2.0 / self.area()
    }

    // fn sample_surface_d(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
    //     let view_dir = (*view_point.0 - self.position);
    //     let d = view_dir.norm();
    //     let r = self.radius;
    //     let normal = math::hs_uniform_sampling(&view_dir.normalize());
    //     let pdf = 1.0 / (2.0 * (PI as Real) * (1.0 - d / (d*d + r*r).sqrt()));
    //     let pos = self.position + (normal * self.radius);

    //     (SurfacePoint {
    //         position: pos,
    //         normal: normal,
    //         bsdf: self.bsdf(),
    //         surface: self,
    //     },
    //     pdf)
    // }

    // fn pdf_d(&self, point_at_surface: (&Point3f, &Vector3f), view_point: (&Point3f, &Vector3f)) -> Real {
    //     let view_dir = (*view_point.0 - *point_at_surface.0);
    //     let d = view_dir.norm();
    //     let r = self.radius;
    //     let pdf = 1.0 / (2.0 * (PI as Real) * (1.0 - d / (d*d + r*r).sqrt()));
    //     pdf
    // }
}

impl HasBounds for Sphere {
    fn aabb(&self) -> Aabb3 {
        let dpos = Vector3f::new(self.radius, self.radius, self.radius);
        Aabb3::new(self.position - dpos, self.position + dpos)
    }
}

impl<'a> Borrow<Surface + 'a> for Sphere {
    #[inline]
    fn borrow(&self) -> &(Surface + 'a) {
        self
    }
}

impl<'a> Borrow<Surface + 'a> for &'a Sphere {
    #[inline]
    fn borrow(&self) -> &(Surface + 'a) {
        *self
    }
}

impl<'a> Borrow<Surface + 'a> for &'a mut Sphere {
    #[inline]
    fn borrow(&self) -> &(Surface + 'a) {
        *self
    }
}

impl<'a> BorrowMut<Surface + 'a> for &'a mut Sphere {
    #[inline]
    fn borrow_mut(&mut self) -> &mut (Surface + 'a) {
        *self
    }
}

impl<'a> AsRef<Surface + 'a> for Sphere {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        self
    }
}

impl<'a> AsMut<Surface + 'a> for Sphere {
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        self
    }
}

impl<'a> AsRef<Surface + 'a> for Box<Sphere> {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        &**self
    }
}

impl<'a> AsMut<Surface + 'a> for Box<Sphere> {
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        &mut **self
    }
}

use std::rc::Rc;

impl<'a> AsRef<Surface + 'a> for Rc<Sphere> {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        &**self
    }
}

impl<'a> AsRef<Surface + 'a> for Arc<Sphere> {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        &**self
    }
}
