pub use self::material::{Material, DiffuseMat, DiffuseTex};
pub use self::vertex::{Vertex, BaseVertex, TexturedVertex, TbnVertex};
use {Surface, SurfacePoint, BsdfRef};
use aabb::{Aabb3, HasBounds};
use color::Color;
use math::{self, Norm, Point3f, Vector3f, Ray3f, Real, Cross};
use rand::{self, Closed01};
use std::marker::PhantomData;

use std::sync::Arc;

pub type PolygonR<'a, R> = Polygon<'a, R, &'a R>;
pub type PolygonS<'a, R> = Polygon<'a, R, R>;

#[derive(Clone)]
pub struct Polygon<'a, R, V = &'a R>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    pub v0: V,
    pub v1: V,
    pub v2: V,
    pub mat: Arc<Material<R> + 'a>,
    total_radiance: Option<Color>,
    _marker: PhantomData<R>,
}

impl<'a, R, V> Polygon<'a, R, V>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    pub fn new(v0: V, v1: V, v2: V, mat: Arc<Material<R> + 'a>) -> Self {
        let e = mat.total_radiance(v0.as_ref(), v1.as_ref(), v2.as_ref());
        Polygon {
            v0: v0,
            v1: v1,
            v2: v2,
            mat: mat,
            total_radiance: e,
            _marker: PhantomData,
        }
    }

    pub fn material(&self, coords: (Real, Real, Real)) -> BsdfRef {
        //self.mat.bsdf(&Vertex::interpolate(self.v0, self.v1, self.v2, coords)) // [FIXME]
        self.mat.bsdf(&Vertex::interpolate(
            self.v0(),
            self.v1(),
            self.v2(),
            coords,
        ))
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

impl<'a, R, V> Surface for Polygon<'a, R, V>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    fn intersection(&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some((t, (u, v))) = math::intersection_triangle(
            &self.v0().position(),
            &self.v1().position(),
            &self.v2().position(),
            ray,
            true,
        ) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.mat
                .normal(self.v0(), self.v1(), self.v2(), (1.0 - u - v, v, u)); // FIXME
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    bsdf: self.material((1.0 - u - v, v, u)),
                    surface: self,
                },
            ))
        } else {
            None
        }
    }

    fn area(&self) -> Real {
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
        unimplemented!()
        //V::normal(self.v0(), self.v1(), self.v2(), p)
    }

    #[inline]
    default fn is_emitter(&self) -> bool {
        self.total_radiance.is_some()
    }

    fn sample_surface_p(&self, (_, _): (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let a = self.v0().position().to_vector();
        let b = self.v1().position().to_vector();
        let c = self.v2().position().to_vector();

        let Closed01(r1) = rand::random::<Closed01<Real>>();
        let Closed01(r2) = rand::random::<Closed01<Real>>();
        let r1s = r1.sqrt();

        //P = (1 − √r1) A + √r1(1 − r2) B + √r1r2 C -- uniform sampling
        let pos = a * (1.0 - r1s) + b * (r1s * (1.0 - r2)) + c * (r1s * r2);
        let w = 1.0 - r1s;
        let u = r1s * (1.0 - r2);
        let v = r1s * r2;
        //let normal = self.normal_at(pos.as_point());
        let normal = self.mat.normal(self.v0(), self.v1(), self.v2(), (w, v, u));

        let pdf = 1.0 / self.area();

        (
            SurfacePoint {
                position: pos.to_point(),
                normal: normal,
                bsdf: self.material((w, u, v)),
                surface: self,
            },
            pdf,
        )
    }

    fn pdf_p(&self, (_, _): (&Point3f, &Vector3f), (_, _): (&Point3f, &Vector3f)) -> Real {
        1.0 / self.area()
    }
}

impl<'a, R, V> HasBounds for Polygon<'a, R, V>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    fn aabb(&self) -> Aabb3 {
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

impl<'a, R, V> AsRef<Surface + 'a> for Polygon<'a, R, V>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        self
    }
}

impl<'a, R, V> AsMut<Surface + 'a> for Polygon<'a, R, V>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        self
    }
}

impl<'a, R, V> AsRef<Surface + 'a> for Box<Polygon<'a, R, V>>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    #[inline]
    fn as_ref(&self) -> &(Surface + 'a) {
        &**self
    }
}

impl<'a, R, V> AsMut<Surface + 'a> for Box<Polygon<'a, R, V>>
where
    R: Vertex + 'a,
    V: AsRef<R> + Sync + Send + Clone + Copy + 'a,
{
    #[inline]
    fn as_mut(&mut self) -> &mut (Surface + 'a) {
        &mut **self
    }
}

pub mod material {
    use super::vertex::{Vertex, BaseVertex, TexturedVertex, TbnVertex};
    use bsdf::{Diffuse, Phong, CookTorrance, BsdfRef};
    use color::{self, Color, Rgb};
    use math;
    use math::{Real, Norm, Cross, Vector3f, Point2f};
    use num::{Float, NumCast};
    use std::marker::PhantomData;
    use std::sync::Arc;
    use texture::{TexView, Texture};
    use utils::consts;

    pub trait Material<V: Vertex>: Sync + Send {
        fn bsdf<'s>(&'s self, v: &V) -> BsdfRef<'s>;

        fn total_radiance(&self, _: &V, _: &V, _: &V) -> Option<Color> {
            None
        }

        fn normal(&self, v0: &V, v1: &V, v2: &V, _: (Real, Real, Real)) -> Vector3f {
            math::triangle_normal(&v0.position(), &v1.position(), &v2.position())
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



    pub struct DiffuseTex<'a, C, T = Texture<C>>
    where
        T: 'a + AsRef<TexView<C> + 'a> + Send + Sync,
        Color: From<C>,
        Rgb<Real>: From<C>,
        C: 'a + Send + Sync,
    {
        pub albedo: T,
        pub radiance: Option<T>,
        _marker_r: PhantomData<&'a (TexView<Color> + 'a)>,
        _marker_c: PhantomData<C>,
    }

    impl<'a, C, T> DiffuseTex<'a, C, T>
    where
        T: 'a + AsRef<TexView<C> + 'a> + Send + Sync,
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

    impl<'a, C, T> Material<TexturedVertex> for DiffuseTex<'a, C, T>
    where
        T: 'a + AsRef<TexView<C> + 'a> + Send + Sync,
        Color: From<C>,
        Rgb<Real>: From<C>,
        C: 'a + Send + Sync,
    {
        fn bsdf<'s>(&'s self, v: &TexturedVertex) -> BsdfRef<'s> {
            let uv = v.uv;
            let albedo = self.albedo.as_ref().sample(uv.x, uv.y);
            let radiance = self.radiance
                .as_ref()
                .map(|e| e.as_ref().sample(uv.x, uv.y).into());
            BsdfRef::Shared(Arc::new(Diffuse::new(albedo.into(), radiance)))
        }

        fn total_radiance(
            &self,
            v0: &TexturedVertex,
            v1: &TexturedVertex,
            v2: &TexturedVertex,
        ) -> Option<Color> {
            if let Some(e_tex) = self.radiance.as_ref() {
                let tex = e_tex.as_ref();
                let u_min = v0.uv[0].min(v1.uv[0].min(v2.uv[0]));
                let u_max = v0.uv[0].max(v1.uv[0].max(v2.uv[0]));
                let v_min = v0.uv[1].min(v1.uv[1].min(v2.uv[1]));
                let v_max = v0.uv[1].max(v1.uv[1].max(v2.uv[1]));

                let u_start = (u_min as Real * tex.width() as Real) as usize;
                let u_finish = (u_max as Real * tex.width() as Real + 0.5) as usize;
                let v_start = (v_min as Real * tex.height() as Real) as usize;
                let v_finish = (v_max as Real * tex.height() as Real + 0.5) as usize;

                let mut sum: Rgb<Real> = color::BLACK.into();
                let mut uv_area: Real = 0.0;
                for v in v_start..v_finish {
                    for u in u_start..u_finish {
                        let t_min = Point2f::new(
                            u as Real / tex.width() as Real,
                            v as Real / tex.height() as Real,
                        );
                        let t_max = Point2f::new(
                            (u + 1) as Real / tex.width() as Real,
                            (v + 1) as Real / tex.height() as Real,
                        );
                        let area = math::intersection_area_tq(
                            Point2f::new(v0.uv.x as Real, v0.uv.y as Real),
                            Point2f::new(v1.uv.x as Real, v1.uv.y as Real),
                            Point2f::new(v2.uv.x as Real, v2.uv.y as Real),
                            t_min,
                            t_max,
                        );
                        if area > 0.0 {
                            let c = (t_min.to_vector() + t_max.to_vector()) * 0.5;
                            let e = tex.pixel(u, v);
                            sum += Rgb::<Real>::from(e) * area;
                            uv_area += area;
                        }
                    }
                }

                let tr_area = math::triangle_area(&v0.position(), &v1.position(), &v2.position());
                let texel_area = tr_area / uv_area;
                sum *= texel_area;

                // println!("texture integral calculated:");
                // println!("  - calc area: {:?}, true area: {:?}", uv_area, tr_area);
                // println!("  - texture total radiance: {:?}", sum);

                Some(sum.into())
            } else {
                None
            }
        }
    }

    pub struct PbrMat {
        pub bsdf: CookTorrance,
    }


    impl PbrMat {
        pub fn new<C, F>(base_color: C, roughness: F, specular: F, metal: F) -> Self
        where
            C: Into<Rgb<Real>>,
            F: Float,
        {
            use utils::clamp;
            use color::ColorClamp;

            let roughness = clamp(<Real as NumCast>::from(roughness).unwrap_or(1.0), 0.0, 1.0);
            let spec = clamp(<Real as NumCast>::from(specular).unwrap_or(1.0), 0.0, 1.0);
            let metal = clamp(<Real as NumCast>::from(metal).unwrap_or(1.0), 0.0, 1.0);
            let c = (base_color.into(): Rgb<Real>).clamp();

            let albedo = c * (1.0 - metal);
            let f0 = c * metal * spec;

            Self {
                bsdf: CookTorrance::new(albedo, f0, roughness * roughness),
            }
        }
    }

    impl<V: Vertex> Material<V> for PbrMat {
        fn bsdf<'s>(&'s self, _: &V) -> BsdfRef<'s> {
            BsdfRef::Ref(&self.bsdf)
        }

        fn total_radiance(&self, _: &V, _: &V, _: &V) -> Option<Color> {
            None
        }
    }


    pub struct PbrTex<'a, C3, C1, Tx3 = Texture<C3>, Tx1 = Texture<C1>>
    where
        Tx3: 'a + AsRef<TexView<C3> + 'a> + Sync + Send,
        Tx1: 'a + AsRef<TexView<C1> + 'a> + Sync + Send,
        C3: Into<Rgb<Real>> + Into<Color>,
        Real: From<C1>,
        C3: 'a + Send + Sync,
        C1: 'a + Send + Sync,
    {
        pub base_color: Tx3,
        pub normal: Tx3,
        pub roughness: Tx1,
        pub specular: Tx1,
        pub metal: Tx1,
        _marker_t3: PhantomData<&'a (TexView<C3> + 'a)>,
        _marker_t1: PhantomData<&'a (TexView<C1> + 'a)>,
        _marker_c3: PhantomData<C3>,
        _marker_c1: PhantomData<C1>,
    }

    impl<'a, C3, C1, Tx3, Tx1> PbrTex<'a, C3, C1, Tx3, Tx1>
    where
        Tx3: 'a + AsRef<TexView<C3> + 'a> + Sync + Send,
        Tx1: 'a + AsRef<TexView<C1> + 'a> + Sync + Send,
        C3: Into<Rgb<Real>> + Into<Color>,
        Real: From<C1>,
        C3: 'a + Send + Sync,
        C1: 'a + Send + Sync,
    {
        pub fn new(
            base_color: Tx3,
            normal: Tx3,
            roughness: Tx1,
            specular: Tx1,
            metal: Tx1,
        ) -> Self {
            Self {
                base_color,
                normal,
                roughness,
                specular,
                metal,
                _marker_t3: PhantomData,
                _marker_t1: PhantomData,
                _marker_c3: PhantomData,
                _marker_c1: PhantomData,
            }
        }
    }

    impl<'a, C3, C1, Tx3, Tx1> Material<TexturedVertex> for PbrTex<'a, C3, C1, Tx3, Tx1>
    where
        Tx3: 'a + AsRef<TexView<C3> + 'a> + Sync + Send,
        Tx1: 'a + AsRef<TexView<C1> + 'a> + Sync + Send,
        C3: Into<Rgb<Real>> + Into<Color>,
        Real: From<C1>,
        C3: 'a + Send + Sync,
        C1: 'a + Send + Sync,
    {
        fn bsdf<'s>(&'s self, v: &TexturedVertex) -> BsdfRef<'s> {
            let basecolor: Rgb<Real> = self.base_color.as_ref().sample(v.uv.x, v.uv.y).into();
            let roughness: Real = self.roughness.as_ref().sample(v.uv.x, v.uv.y).into();
            let spec: Real = self.specular.as_ref().sample(v.uv.x, v.uv.y).into();
            let metal: Real = self.metal.as_ref().sample(v.uv.x, v.uv.y).into();

            let albedo = basecolor * (1.0 - metal);
            let f0 = basecolor * metal * spec;

            BsdfRef::Shared(Arc::new(
                CookTorrance::new(albedo, f0, roughness * roughness),
            ))
        }

        fn total_radiance(
            &self,
            _: &TexturedVertex,
            _: &TexturedVertex,
            _: &TexturedVertex,
        ) -> Option<Color> {
            None
        }

        fn normal(
            &self,
            v0: &TexturedVertex,
            v1: &TexturedVertex,
            v2: &TexturedVertex,
            wuv: (Real, Real, Real),
        ) -> Vector3f {
            let p = Vertex::interpolate(v0, v1, v2, wuv);
            let mut rgb_normal: Rgb<Real> = self.normal.as_ref().sample(p.uv.x, p.uv.y).into();
            rgb_normal = rgb_normal * 2.0 - Rgb::<Real>::from(1.0);
            let tex_normal = Vector3f::new(rgb_normal.r, rgb_normal.g, rgb_normal.b).normalize();

            let duv1 = v1.uv - v0.uv;
            let duv2 = v2.uv - v0.uv;
            let (t, b) = math::calc_tangent(
                (&(v1.position - v0.position), duv1.x as Real, duv1.y as Real),
                (&(v2.position - v0.position), duv2.x as Real, duv2.y as Real),
            );

            let n = t.cross(&b).normalize();

            let normal = Vector3f::new(
                tex_normal.x * t.x + tex_normal.y * b.x + tex_normal.z * n.x,
                tex_normal.x * t.y + tex_normal.y * b.y + tex_normal.z * n.y,
                tex_normal.x * t.z + tex_normal.y * b.z + tex_normal.z * n.z,
            );

            normal.normalize()
        }
    }

    impl<'a, C3, C1, Tx3, Tx1> Material<TbnVertex> for PbrTex<'a, C3, C1, Tx3, Tx1>
    where
        Tx3: 'a + AsRef<TexView<C3> + 'a> + Sync + Send,
        Tx1: 'a + AsRef<TexView<C1> + 'a> + Sync + Send,
        C3: Into<Rgb<Real>> + Into<Color>,
        Real: From<C1>,
        C3: 'a + Send + Sync,
        C1: 'a + Send + Sync,
    {
        fn bsdf<'s>(&'s self, v: &TbnVertex) -> BsdfRef<'s> {
            let basecolor: Rgb<Real> = self.base_color.as_ref().sample(v.uv.x, v.uv.y).into();
            let roughness: Real = self.roughness.as_ref().sample(v.uv.x, v.uv.y).into();
            let spec: Real = self.specular.as_ref().sample(v.uv.x, v.uv.y).into();
            let metal: Real = self.metal.as_ref().sample(v.uv.x, v.uv.y).into();

            let albedo = basecolor * (1.0 - metal);
            let f0 = basecolor * metal * spec;

            BsdfRef::Shared(Arc::new(
                CookTorrance::new(albedo, f0, roughness * roughness),
            ))
        }

        fn total_radiance(&self, _: &TbnVertex, _: &TbnVertex, _: &TbnVertex) -> Option<Color> {
            None
        }

        fn normal(
            &self,
            v0: &TbnVertex,
            v1: &TbnVertex,
            v2: &TbnVertex,
            wuv: (Real, Real, Real),
        ) -> Vector3f {
            let p = Vertex::interpolate(v0, v1, v2, wuv);
            let mut rgb_normal: Rgb<Real> = self.normal.as_ref().sample(p.uv.x, p.uv.y).into();
            rgb_normal = rgb_normal * 2.0 - Rgb::<Real>::from(1.0);
            let tex_normal = Vector3f::new(rgb_normal.r, rgb_normal.g, rgb_normal.b).normalize();

            // let duv1 = v1.uv - v0.uv;
            // let duv2 = v2.uv - v0.uv;
            // let (t, b) = math::calc_tangent(
            //     (&(v1.position - v0.position), duv1.x as Real, duv1.y as Real),
            //     (&(v2.position - v0.position), duv2.x as Real, duv2.y as Real));

            // let n = t.cross(&b).normalize();

            let t = p.tangent;
            let b = p.bitangent;
            let n = p.normal;

            let normal = Vector3f::new(
                tex_normal.x * t.x + tex_normal.y * b.x + tex_normal.z * n.x,
                tex_normal.x * t.y + tex_normal.y * b.y + tex_normal.z * n.y,
                tex_normal.x * t.z + tex_normal.y * b.z + tex_normal.z * n.z,
            );

            normal.normalize()
        }
    }
}

pub mod vertex {
    use math::{Vector3f, Vector2, Point3f, Real};
    use math::Norm;

    pub trait Vertex: Copy + Clone + Sync + Send {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, p: (Real, Real, Real)) -> Self
        where
            Self: Sized;
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
        pub fn new(pos: Point3f) -> BaseVertex {
            BaseVertex { position: pos }
        }
    }
    impl_asref_for_vertex!(BaseVertex);

    impl Vertex for BaseVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self
        where
            Self: Sized,
        {
            let pos = v0.position.to_vector() * w + v1.position.to_vector() * u +
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
        pub uv: Vector2<f32>,
    }

    impl TexturedVertex {
        pub fn new(position: Point3f, uv: Vector2<f32>) -> Self {
            Self { position, uv }
        }
    }

    impl_asref_for_vertex!(TexturedVertex);

    impl Vertex for TexturedVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self {
            let pos = v0.position.to_vector() * w + v1.position.to_vector() * u +
                v2.position.to_vector() * v;
            let tex_uv = v0.uv * (w as f32) + v1.uv * (u as f32) + v2.uv * (v as f32);

            TexturedVertex::new(pos.to_point(), tex_uv)
        }

        #[inline]
        fn position(&self) -> Point3f {
            self.position
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C)]
    pub struct TbnVertex {
        pub position: Point3f,
        pub tangent: Vector3f,
        pub bitangent: Vector3f,
        pub normal: Vector3f,
        pub uv: Vector2<f32>,
    }

    impl TbnVertex {
        pub fn new(
            position: Point3f,
            tangent: Vector3f,
            bitangent: Vector3f,
            normal: Vector3f,
            uv: Vector2<f32>,
        ) -> Self {
            Self {
                position,
                tangent,
                bitangent,
                normal,
                uv,
            }
        }
    }

    impl_asref_for_vertex!(TbnVertex);

    impl Vertex for TbnVertex {
        fn interpolate(v0: &Self, v1: &Self, v2: &Self, (w, u, v): (Real, Real, Real)) -> Self {
            let pos = v0.position.to_vector() * w + v1.position.to_vector() * u +
                v2.position.to_vector() * v;
            let uv = v0.uv * (w as f32) + v1.uv * (u as f32) + v2.uv * (v as f32);

            let t = (v0.tangent * w + v1.tangent * u + v2.tangent * v).normalize();
            let b = (v0.bitangent * w + v1.bitangent * u + v2.bitangent * v).normalize();
            let n = (v0.normal * w + v1.normal * u + v2.normal * v).normalize();

            TbnVertex::new(pos.to_point(), t, b, n, uv)
        }

        #[inline]
        fn position(&self) -> Point3f {
            self.position
        }
    }
}
