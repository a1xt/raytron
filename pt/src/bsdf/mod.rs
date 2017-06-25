pub mod diffuse;
pub mod phong;
pub mod cooktorrance;

pub use self::diffuse::Diffuse;
pub use self::phong::Phong;
pub use self::cooktorrance::*;

use color::Color;
use math::{Vector3f, Real, Dot};
use std::sync::Arc;
use std::ops::Deref;

pub trait Bsdf : Sync + Send {

    fn radiance(&self) -> Option<Color>;

    fn eval(&self, 
            surface_normal: &Vector3f,
            in_dir: &Vector3f,
            out_dir: &Vector3f)
            -> (Color, Real);

    fn sample(&self, 
              surface_normal: &Vector3f, 
              in_dir: &Vector3f)
              -> (Vector3f, Color, Real);

    fn eval_proj(&self, 
                 surface_normal: &Vector3f, 
                 in_dir: &Vector3f,
                 out_dir: &Vector3f)
                 -> (Color, Real) {

        let (fr, pdf) = self.eval(surface_normal, in_dir, out_dir);
        let cos_theta = surface_normal.dot(&out_dir);
        (fr, pdf / cos_theta)
    }  

    fn sample_proj(&self, 
                   surface_normal: &Vector3f, 
                   in_dir: &Vector3f)
                   -> (Vector3f, Color, Real) {

        let (ray, fr, pdf) = self.sample(surface_normal, in_dir);
        let cos_theta = surface_normal.dot(&ray);

        (ray, fr, pdf / cos_theta)
    }

}

pub enum BsdfRef<'a> {
    Ref(&'a Bsdf),
    Shared(Arc<Bsdf + 'a>),
}

impl<'a> Deref for BsdfRef<'a> {
    type Target = Bsdf + 'a;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            &BsdfRef::Ref(r) => r,
            &BsdfRef::Shared(ref rc) => rc.as_ref(),
        }
    }
}

impl<'a> AsRef<Bsdf + 'a> for BsdfRef<'a> {
    #[inline]
    fn as_ref(&self) -> &(Bsdf + 'a) {
        &**self
    }
}