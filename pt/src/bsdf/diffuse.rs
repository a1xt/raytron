use {Bsdf};
use math::{self, Vector3f, Real, Dot};
use color::{self, Color, Rgb};
use std::f64::consts::PI;

#[inline]
pub fn sample<F>(normal: &Vector3f, albedo: &Rgb<F>) -> (Vector3f, Rgb<F>, Real) 
    where Color: From<Rgb<F>>,
          Rgb<F>: From<F>,
          F: color::ColorChannel + color::ChannelCast<Real>,
{
    let vec_out = math::hs_cosine_sampling(normal);
    let cos_no = normal.dot(&vec_out);
    let pdf = cos_no;
    let fr: Rgb<F> = *albedo * Rgb::from(F::cast_from(1.0 as Real));
    (vec_out, fr, pdf)
}
 
#[inline]
pub fn eval<F>(cos_no: Real, albedo: &Rgb<F>) -> (Rgb<F>, Real)
    where Color: From<Rgb<F>>,
          Rgb<F>: From<F>,
          F: color::ColorChannel + color::ChannelCast<Real>,
{
    let pdf = cos_no / PI as Real;
    let fr: Rgb<F> = *albedo * Rgb::from(F::cast_from(1.0 as Real / PI as Real));
    (fr, pdf)
}

#[derive(Clone, Copy, Debug)]
pub struct Diffuse {
    pub color: Color,
    pub radiance: Option<Color>,
}

impl Diffuse {
    pub fn new(color: Color, radiance: Option<Color>) -> Diffuse {
        Diffuse {
            color: color,
            radiance: radiance,
        }
    }
}

impl Bsdf for Diffuse {

    fn radiance(&self) -> Option<Color> {
        self.radiance
    }

    fn sample(
        &self, 
        surface_normal: &Vector3f, 
        _: &Vector3f)
        -> (Vector3f, Color, Real)
    {
        sample::<f32>(surface_normal, &self.color)
    }

    // fn sample_proj(
    //     &self, 
    //     surface_normal: &Vector3f, 
    //     _: &Vector3f)
    //     -> (Vector3f, Color, Real)
    // {
    //     let out_dir = math::hs_cosine_sampling(surface_normal);
    //     (out_dir, self.color, 1.0)
    // }

    fn eval(
        &self, 
        surface_normal: &Vector3f,
        _: &Vector3f,
        out_dir: &Vector3f)
        -> (Color, Real)
    {
        let cos_no = surface_normal.dot(out_dir);
        eval::<f32>(cos_no, &self.color)
    }

    // fn eval_proj(
    //     &self, 
    //     _: &Vector3f,
    //     _: &Vector3f, 
    //     _: &Vector3f)
    //     -> (Color, Real)
    // {
    //     let reflectance = self.color * (1.0 / PI as f32);
    //     let pdf = 1.0 / PI as Real;

    //     (reflectance, pdf)
    // }
}
