pub use self::vertex::{Vertex, BaseVertex, TexturedVertex};
pub use self::material::{Material, DiffuseMat, DiffuseTex};

use std::sync::Arc;
use std::marker::PhantomData;
use math::{self, Norm, Point3f, Vector3f, Ray3f, Real, Cross};
use {Surface, SurfacePoint, BsdfRef};
use aabb::{Aabb3, HasBounds};

use color::{self, Color};
use rand::{self, Closed01};
use num::Float;

pub type PolygonR<'a, R> = Polygon<'a, R, &'a R>;
pub type PolygonS<'a, R> = Polygon<'a, R, R>;

#[derive(Clone)]
pub struct Polygon<'a, R, V = &'a R> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a, {
    pub v0: V,
    pub v1: V,
    pub v2: V,
    pub mat: Arc<Material<R> + 'a>,
    total_radiance: Option<Color>,
    _marker: PhantomData<R>,
}

impl<'a, R, V> Polygon<'a, R, V> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    pub fn new (v0: V, v1: V, v2: V, mat: Arc<Material<R> + 'a>) -> Self {
        let e = mat.total_radiance(v0.as_ref(), v2.as_ref(), v1.as_ref());
        Polygon {
            v0: v0,
            v1: v1,
            v2: v2,
            mat: mat,
            total_radiance: e,
            _marker: PhantomData,
        }
    }

    pub fn material<'s> (&'s self, coords: (Real, Real, Real)) -> BsdfRef<'s> {
        //self.mat.bsdf(&Vertex::interpolate(self.v0, self.v1, self.v2, coords)) // [FIXME]
        self.mat.bsdf(&Vertex::interpolate(self.v0(), 
                                           self.v2(), 
                                           self.v1(), 
                                           coords))
    }

    #[inline]
    pub fn v0(&self) -> &R {
        self.v0.as_ref()
    }

    #[inline]
    pub fn v1(&self) -> &R {
        self.v1.as_ref()
    }

    #[inline]
    pub fn v2(&self) -> &R {
        self.v2.as_ref()
    }

}

impl<'a, R, V> Surface for Polygon<'a, R, V> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    fn intersection (&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some((t, (u, v))) = math::intersection_triangle(&self.v0().position(), 
                                                               &self.v1().position(), 
                                                               &self.v2().position(), 
                                                               ray, 
                                                               true)
        {
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
        let a = self.v1().position() - self.v0().position();
        let b = self.v2().position() - self.v0().position();
        0.5 * b.cross(&a).norm()
    }

    #[inline]
    default fn total_radiance(&self) -> Option<Color> {
        self.total_radiance
    }

    #[inline]
    default fn normal_at(&self, _: &Point3f) -> Vector3f {
        let a = self.v1().position() - self.v0().position();
        let b = self.v2().position() - self.v0().position();
        b.cross(&a).normalize()
    }

    #[inline]
    default fn is_emitter(&self) -> bool {
        if let Some(_) = self.total_radiance {
            true
        } else {
            false
        }
    }

    fn sample_surface_p(&self, (_, _): (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let a = self.v0().position().to_vector();
        let b = self.v1().position().to_vector();
        let c = self.v2().position().to_vector();

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
}

impl<'a, R, V> HasBounds for Polygon<'a, R, V> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {  
    fn aabb(&self) -> Aabb3 {
        use utils::consts::POSITION_EPSILON;
        let p0 = self.v0().position();
        let p1 = self.v1().position();
        let p2 = self.v2().position();
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

impl<'a, R, V> AsRef<Surface + 'a> for Polygon<'a, R, V> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        self
    }
}

impl<'a, R, V> AsMut<Surface + 'a> for Polygon<'a, R, V> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        self
    }
}

impl<'a, R, V> AsRef<Surface + 'a> for Box<Polygon<'a, R, V>> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        &**self
    }
}

impl<'a, R, V> AsMut<Surface + 'a> for Box<Polygon<'a, R, V>> where R: Vertex + 'a, V: AsRef<R> + Sync + Clone + Copy + 'a {
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        &mut **self
    }
}

pub mod material {
    use bsdf::{Diffuse, Phong, BsdfRef};
    use super::vertex::{Vertex, BaseVertex, TexturedVertex};
    use color::{self, Color, Rgb, ColorChannel};
    use texture::{Tex, Texture};
    use std::sync::Arc;
    use math;
    use math::{Real, Norm};
    use utils::consts;
    use num::Float;

    pub trait Material<V: Vertex>: Sync + Send {
        fn bsdf<'s>(&'s self, v: &V) -> BsdfRef<'s>;

        fn total_radiance(&self, v0: &V, v1: &V, v2: &V) -> Option<Color> {
            None
        }
    }

    pub struct DiffuseMat {
        pub bsdf: Diffuse,
    }

    impl DiffuseMat {
        pub fn new(color: Color, radiance: Option<Color>) -> DiffuseMat {
            DiffuseMat {
                bsdf: Diffuse::new(color, radiance),
            }
        }
    }

    impl<V: Vertex> Material<V> for DiffuseMat {
        fn bsdf<'s>(&'s self, _: &V) -> BsdfRef<'s> {
            BsdfRef::Ref(&self.bsdf)
        }

        fn total_radiance(&self, v0: &V, v1: &V, v2: &V) -> Option<Color> {
            if let Some(e) = self.bsdf(v0).radiance() {
                let area = math::triangle_area(&v0.position(), &v1.position(), &v2.position());
                Some(e * (area as f32))
            } else {
                None
            }
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
    use std::marker::PhantomData;
    pub struct DiffuseTex<'a, T, C>
        where T: 'a + AsRef<Tex<C> + 'a> + Send + Sync,
              Color: From<C>,
              Rgb<Real>: From<C>,
              C: 'a + Send + Sync,
    {
        pub albedo: T,
        pub radiance: Option<T>,
        _marker_r: PhantomData<&'a (Tex<Color> + 'a)>,
        _marker_c: PhantomData<C>,
    }

    impl<'a, T, C> DiffuseTex<'a, T, C> 
        where T: 'a + AsRef<Tex<C> + 'a> + Send + Sync,
              Color: From<C>,
              Rgb<Real>: From<C>,
              C: 'a + Send + Sync,
    {
        pub fn new(albedo: T, radiance: Option<T>) -> Self {
            Self {
                albedo,
                radiance,
                _marker_r: PhantomData,
                _marker_c: PhantomData,
            }
        }
    }

    impl<'a, T, C> Material<TexturedVertex> for DiffuseTex<'a, T, C>
        where T: 'a + AsRef<Tex<C> + 'a> + Send + Sync,
              Color: From<C>,
              Rgb<Real>: From<C>,
              C: 'a + Send + Sync,
    {
        fn bsdf<'s>(&'s self, v: &TexturedVertex) -> BsdfRef<'s> {
            let uv = v.uv;
            let albedo = self.albedo.as_ref().sample(uv.x, uv.y);
            let radiance = self.radiance.as_ref().map(|e| e.as_ref().sample(uv.x, uv.y).into());
            BsdfRef::Shared(Arc::new(Diffuse::new(albedo.into(), radiance)))
        }

        fn total_radiance(&self, v0: &TexturedVertex, v1: &TexturedVertex, v2: &TexturedVertex) -> Option<Color> {
            if let Some(e_tex) = self.radiance.as_ref() {
                let dt = consts::TEXTURE_INTEGRAL_STEP;
                let da: Real = 2.0 * (math::triangle_area(&v0.position(), &v1.position(), &v2.position()) * dt * dt);
                let mut sum: Rgb<Real> = color::BLACK.into();
                let mut u = dt;
                let mut v = dt;
                let mut cl_area = 0.0;
                while u < 1.0 + consts::REAL_EPSILON {
                    v = dt;
                    while v < 1.0 +consts::REAL_EPSILON {
                        if u + v < 1.0 + consts::REAL_EPSILON {
                            let us = u - dt * 0.25;
                            let vs = v - dt * 0.25;
                            let p = <TexturedVertex as Vertex>::interpolate(v0, v1, v2, (1.0 - us - vs, us, vs));
                            let e = e_tex.as_ref().sample(p.uv.x, p.uv.y);
                            sum += Into::<Rgb<Real>>::into(e) * (0.5 * da);
                            cl_area += 0.5 * da;
                        }
                        if u + v - dt < 1.0 + consts::REAL_EPSILON {
                            let us = u - dt * 0.75;
                            let vs = v - dt * 0.75;
                            let p = <TexturedVertex as Vertex>::interpolate(v0, v1, v2, (1.0 - us - vs, us, vs));
                            let e = e_tex.as_ref().sample(p.uv.x, p.uv.y);
                            sum += Into::<Rgb<Real>>::into(e) * (0.5 * da);
                            cl_area += 0.5 * da;
                        }
                        v += dt;
                    }
                    u += dt;
                }
                let area = math::triangle_area(&v0.position(), &v1.position(), &v2.position());
                println!("texture integral calculated:");
                println!("  - calc area: {:?}, true area: {:?}", cl_area, area);
                println!("  - texture total radiance: {:?}", sum);
                Some(sum.into())
            } else {
                None
            }
        }
    }
}

pub mod vertex {
    use math::{Vector3f, Point2, Point3f, Real};
    use color::{Color};

    pub trait Vertex: Copy + Clone + Sync + Send {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, p: (Real, Real, Real)) -> Self where Self: Sized;
        //fn normal(v0: &Self, v1: &Self, v2: &Self, p: (Real, Real, Real)) -> Vector3f;
        fn position(&self) -> Point3f;
    }

    macro_rules! impl_asref_for_vertex {
        ($type:ident) => {
            impl AsRef<$type> for $type {
                fn as_ref(&self) -> & $type {
                    self
                }
            }

            impl AsMut<$type> for $type {
                fn as_mut(&mut self) -> &mut $type {
                    self
                }
            }
        }
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
    impl_asref_for_vertex!(BaseVertex);

    impl Vertex for BaseVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self where Self: Sized {
            let pos = v0.position.to_vector() * w + 
                      v1.position.to_vector() * u + 
                      v2.position.to_vector() * v;
            BaseVertex::new(pos.to_point())
        }

        #[inline]
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

    impl_asref_for_vertex!(TexturedVertex);

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

        #[inline]
        fn position(&self) -> Point3f {
            self.position
        }
    }
}