pub use self::vertex::{Vertex, BaseVertex, TexturedVertex};
pub use self::material::{Material, DiffuseMat, DiffuseTex};

use std::sync::Arc;
use math::{self, Norm, Point3f, Vector3f, Ray3f, Real, Cross};
use {Surface, SurfacePoint, BsdfRef};
use aabb::{Aabb3, HasBounds};

use color::{self, Color};
use rand::{self, Closed01};
use num::Float;

#[derive(Clone)]
pub struct Polygon<'a, V: Vertex + 'a> {
    pub v0: &'a V,
    pub v1: &'a V,
    pub v2: &'a V,
    pub mat: Arc<Material<V> + 'a>,
}

impl<'a, V: Vertex + 'a> Polygon<'a, V> {
    pub fn new (v0: &'a V, v1: &'a V, v2: &'a V, mat: Arc<Material<V> + 'a>) -> Self {
        Polygon {
            v0: v0,
            v1: v1,
            v2: v2,
            mat: mat,
        }
    }

    pub fn material<'s> (&'s self, coords: (Real, Real, Real)) -> BsdfRef<'s> {
        //self.mat.bsdf(&Vertex::interpolate(self.v0, self.v1, self.v2, coords)) // [FIXME]
        self.mat.bsdf(&Vertex::interpolate(self.v0, self.v2, self.v1, coords))
    }

}

impl<'a, V: Vertex + 'a> Surface for Polygon<'a, V> {
    fn intersection (&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some((t, (u, v))) = math::intersection_triangle(&self.v0.position(), &self.v1.position(), &self.v2.position(), ray, true) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_at(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    bsdf: self.material((1.0 - u - v, u, v)),
                    surface: self,
                }
            ))
        } else {
            None
        }
    }

    fn area (&self) -> Real {
        let a = self.v1.position() - self.v0.position();
        let b = self.v2.position() - self.v0.position();
        0.5 * b.cross(&a).norm()
    }

    fn total_emittance(&self) -> Option<Color> {
        if let Some(e) = self.mat.bsdf(self.v0).emittance() {
            Some(e * (self.area() as f32))
        } else {
            None
        }
    }

    fn normal_at(&self, _: &Point3f) -> Vector3f {
        let a = self.v1.position() - self.v0.position();
        let b = self.v2.position() - self.v0.position();
        b.cross(&a).normalize()
    }

    fn sample_surface_p(&self, (_, _): (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let a = self.v0.position().to_vector();
        let b = self.v1.position().to_vector();
        let c = self.v2.position().to_vector();

        let Closed01(r1) = rand::random::<Closed01<Real>>();
        let Closed01(r2) = rand::random::<Closed01<Real>>();
        let r1s = r1.sqrt();
        
        //P = (1 − √r1) A + √r1(1 − r2) B + √r1r2 C
        let pos = a * (1.0 - r1s) + b * (r1s * (1.0 - r2)) + c * (r1s * r2);
        let w = 1.0 - r1s;
        let u = r1s * (1.0 - r2);
        let v = r1s * r2;
        let normal = self.normal_at(pos.as_point());

        let pdf = 1.0 / self.area();

        (SurfacePoint {
            position: pos.to_point(),
            normal: normal,
            bsdf: self.material((w, u, v)),
            surface: self,
        },
        pdf)
    }

    fn pdf_p(&self, (_, _): (&Point3f, &Vector3f), (_, _): (&Point3f, &Vector3f)) -> Real {
        1.0 / self.area()
    }

    fn is_emitter(&self) -> bool {
        if let Some(_) = self.mat.bsdf(self.v0).emittance() {
            true
        } else {
            false
        }
    }
}

impl<'a, V: Vertex> HasBounds for Polygon<'a, V> {  
    fn aabb(&self) -> Aabb3 {
        use utils::consts::POSITION_EPSILON;
        let p0 = self.v0.position();
        let p1 = self.v1.position();
        let p2 = self.v2.position();
        let pmin = Point3f::new(
            p0.x.min(p1.x.min(p2.x)),
            p0.y.min(p1.y.min(p2.y)),
            p0.z.min(p1.z.min(p2.z)),
        );
        let pmax = Point3f::new(
            p0.x.max(p1.x.max(p2.x)),
            p0.y.max(p1.y.max(p2.y)),
            p0.z.max(p1.z.max(p2.z)),
        );

        Aabb3::new(pmin, pmax)
    }
}

pub mod material {
    use bsdf::{Diffuse, Phong, BsdfRef};
    use super::vertex::{Vertex, BaseVertex, TexturedVertex};
    use color::{Color, Rgb, ColorChannel};
    use texture::Texture;
    use std::sync::Arc;

    pub trait Material<V: Vertex>: Sync + Send {
        fn bsdf<'s>(&'s self, v: &V) -> BsdfRef<'s>;
    }

    pub struct DiffuseMat {
        pub bsdf: Diffuse,
    }

    impl DiffuseMat {
        pub fn new(color: Color, emittance: Option<Color>) -> DiffuseMat {
            DiffuseMat {
                bsdf: Diffuse::new(color, emittance),
            }
        }
    }

    impl Material<BaseVertex> for DiffuseMat {
        fn bsdf<'s>(&'s self, _: &BaseVertex) -> BsdfRef<'s> {
            BsdfRef::Ref(&self.bsdf)
        }
    }

    pub struct PhongMat {
        pub bsdf: Phong,
    }

    impl PhongMat {
        pub fn new(color: Color, kd: f32, ks: f32, n: f32) -> PhongMat {
            PhongMat {
                bsdf: Phong::new(color, kd, ks, n),
            }
        }
    }

    impl Material<BaseVertex> for PhongMat {
        fn bsdf<'s>(&'s self, _: &BaseVertex) -> BsdfRef<'s> {
            BsdfRef::Ref(&self.bsdf)
        }
    }

    pub struct DiffuseTex<'a, T = f32> where T: 'a + ColorChannel, Rgb<T>: Into<Color> {
        pub albedo: &'a Texture<Rgb<T>, [T; 4]>,
    }

    impl<'a, T> DiffuseTex<'a, T> where T: 'a + ColorChannel, Rgb<T>: Into<Color> {
        pub fn new<'b: 'a>(albedo_texture: &'b Texture<Rgb<T>, [T; 4]>) -> Self {
            Self {albedo: albedo_texture}
        }
    }

    impl<'a, T> Material<TexturedVertex> for DiffuseTex<'a, T> where T: 'a + ColorChannel, Rgb<T>: Into<Color> {
        fn bsdf<'s>(&'s self, v: &TexturedVertex) -> BsdfRef<'s> {
            let uv = v.uv;
            let albedo = self.albedo.sample(uv[0], uv[1]);
            BsdfRef::Shared(Arc::new(Diffuse::new(albedo.into(), None)))
        }
    }
}

pub mod vertex {
    use math::{Vector3f, Point2, Point3f, Real};

    pub trait Vertex: Sync + Send {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, p: (Real, Real, Real)) -> Self;
        //fn normal(v0: &Self, v1: &Self, v2: &Self, p: (Real, Real, Real)) -> Vector3f;
        fn position(&self) -> Point3f;
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C)]
    pub struct BaseVertex {
        pub position: Point3f,
    }

    impl BaseVertex {
        pub fn new (pos: Point3f) -> BaseVertex {
            BaseVertex {
                position: pos,
            }
        }
    }

    impl Vertex for BaseVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self {
            let pos = v0.position.to_vector() * w + 
                      v1.position.to_vector() * u + 
                      v2.position.to_vector() * v;
            BaseVertex::new(pos.to_point())
        }

        fn position(&self) -> Point3f {
            self.position
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C)]
    pub struct TexturedVertex {
        pub position: Point3f,
        pub uv: Point2<f32>,
    }

    impl TexturedVertex {
        pub fn new(pos: Point3f, uv: Point2<f32>) -> Self {
            Self {
                position: pos,
                uv: uv,
            }
        }
    }

    impl Vertex for TexturedVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self {
            let pos = v0.position.to_vector() * w + 
                      v1.position.to_vector() * u + 
                      v2.position.to_vector() * v;
            let tex_uv = v0.uv.to_vector() * (w as f32) + 
                         v1.uv.to_vector() * (u as f32) +
                         v2.uv.to_vector() * (v as f32);
            TexturedVertex::new(pos.to_point(), tex_uv.to_point())
        }

        fn position(&self) -> Point3f {
            self.position
        }
    }
}