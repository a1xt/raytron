pub mod shapelist;
pub mod kdtree;

pub use self::shapelist::{ShapeList, ShapeListBuilder};
pub use self::kdtree::{KdTree, KdTreeS};
pub use self::kdtree::{KdTreeSetup, Sah};

use traits::Surface;
use math::{Real, Point3f, Vector3f, Ray3f};
use {SurfacePoint};
use std::sync::Arc;
use rand;

pub trait SceneHandler: Sync {
    fn intersection(&self, ray: &Ray3f) -> Option<SurfacePoint>;
    
    fn light_sources_iter<'s>(&'s self) -> Box<Iterator<Item = &'s Surface> + 's>;
    fn light_sources<'s>(&'s self) -> LightSourcesHandler<'s>;
}

pub trait LuminairesSampler<'a>: Sync + Send {
     fn sample(&self, 
               view_point: (&Point3f, &Vector3f), 
               surface_sampler: fn(&'a Surface, (&Point3f, &Vector3f)) -> (SurfacePoint<'a>, Real)) 
               -> Option<(SurfacePoint<'a>, Real)>;

    fn pdf(&self, 
           surface: &'a Surface,
           point_at_surface: (&Point3f, &Vector3f), 
           view_point: (&Point3f, &Vector3f),
           surface_pdf: fn(&'a Surface, (&Point3f, &Vector3f), (&Point3f, &Vector3f)) -> Real) 
           -> Real;
}

#[derive(Clone)]
pub struct UniformSampler<'s> {
    surfaces: Vec<&'s Surface>,
}

impl<'a> LuminairesSampler<'a> for UniformSampler<'a> {
    fn sample(&self, 
              view_point: (&Point3f, &Vector3f), 
              surface_sampler: fn(&'a Surface, (&Point3f, &Vector3f)) -> (SurfacePoint<'a>, Real)) 
              -> Option<(SurfacePoint<'a>, Real)>
    {
        let s_num = self.surfaces.len();

        if s_num > 0 {
            let i = rand::random::<usize>() % s_num;
            let s = self.surfaces[i];
            let (sp, pdf) = surface_sampler(s, view_point);

            Some((sp, pdf / s_num as Real))
        } else {
            None
        }

    }

    fn pdf(&self, 
           surface: &'a Surface,
           point_at_surface: (&Point3f, &Vector3f), 
           view_point: (&Point3f, &Vector3f),
           surface_pdf: fn(&'a Surface, (&Point3f, &Vector3f), (&Point3f, &Vector3f)) -> Real) 
           -> Real
    {
        let s_num = self.surfaces.len();
        let pdf = surface_pdf(surface, point_at_surface, view_point);

        pdf / s_num as Real
    }
}

impl<'s, 'a> From<&'s [&'a Surface]> for UniformSampler<'a> {
    fn from(other: &'s [&'a Surface]) -> Self {
        Self {
            surfaces: other.to_vec(),
        }
    }
}

pub struct LinearSampler<'a> {
    surfaces: Vec<&'a Surface>,
    partial_sum: Vec<Real>,
    sum: Real,
}

use color::{Color, Rgb, ColorChannel};
fn color_norm<T: ColorChannel>(c: &Rgb<T>) -> Real where Rgb<Real>: From<Rgb<T>> {
    let c: Rgb<Real> = Rgb::from(*c);
    (c.r * c.r + c.g * c.g + c.b * c.b).sqrt() as Real
}

impl<'s, 'a> From<&'s [&'a Surface]> for LinearSampler<'a> {
    fn from(other: &'s [&'a Surface]) -> Self {
        let mut surfaces = other.to_vec();
        surfaces.sort_by(|&s1, &s2| {
            let c1 = color_norm(&s1.total_radiance().unwrap());
            let c2 = color_norm(&s2.total_radiance().unwrap());
            c1.partial_cmp(&c2).unwrap()
        });

        let mut sum = 0.0;
        let partial_sum = 
            surfaces
            .iter()
            .map(|&s|{ 
                let e = color_norm(&s.total_radiance().unwrap()) as Real;
                sum += e;
                sum
            })
            .collect();

        Self {
            surfaces,
            partial_sum,
            sum
        }
    }
}
 
impl<'a> LuminairesSampler<'a> for LinearSampler<'a> {
    fn sample(&self, 
              view_point: (&Point3f, &Vector3f), 
              surface_sampler: fn(&'a Surface, (&Point3f, &Vector3f)) -> (SurfacePoint<'a>, Real)) 
              -> Option<(SurfacePoint<'a>, Real)>
    {
        use rand::Closed01;
        let s_num = self.surfaces.len();

        if s_num > 0 {
            let Closed01(mut e) = rand::random::<Closed01<Real>>();
            e *= self.sum;

            let ix = match self.partial_sum.binary_search_by(|&probe| probe.partial_cmp(&e).unwrap()) {
                Ok(x) => x,
                Err(x) => x,
            };

            let s = self.surfaces[ix];
            let il = color_norm(&s.total_radiance().unwrap());
            let pdf = il / self.sum;
            let (sp, spdf) = surface_sampler(s, view_point);

            Some((sp, spdf * pdf))

        } else {

            None
        }

    }

    fn pdf(&self, 
           surface: &'a Surface,
           point_at_surface: (&Point3f, &Vector3f), 
           view_point: (&Point3f, &Vector3f),
           surface_pdf: fn(&'a Surface, (&Point3f, &Vector3f), (&Point3f, &Vector3f)) -> Real) 
           -> Real
    {
        let pdf = surface_pdf(surface, point_at_surface, view_point);
        let il = color_norm(&surface.total_radiance().unwrap());
        let spdf = il / self.sum;

        pdf * spdf
    }
}

#[derive(Clone)]
pub struct LightSourcesIter<'a> {
    scene: &'a SceneHandler,
}

impl<'a> IntoIterator for LightSourcesIter<'a> {
    type Item = &'a Surface;
    type IntoIter = Box<Iterator<Item = &'a Surface> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.scene.light_sources_iter()
    }
}


pub struct LightSourcesHandler<'a> {
    scene: &'a SceneHandler,
    sampler: Arc<LuminairesSampler<'a> + 'a>,
}

impl<'a> LightSourcesHandler<'a> {
    pub fn iter(&self) -> LightSourcesIter<'a> {
        LightSourcesIter {
            scene: self.scene,
        }
    }
}

use std::ops::Deref;
impl<'a> Deref for LightSourcesHandler<'a> {
    type Target = LuminairesSampler<'a> + 'a;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.sampler.as_ref()
    }
}
#[inline]
fn lt_arc_trait_hack<'a, 'b: 'a>(t: Arc<LuminairesSampler<'b> + 'b>) -> Arc<LuminairesSampler<'a> + 'b> {
    unsafe {::std::mem::transmute(t)}
}