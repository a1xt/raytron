use math::{Vector3f, Real, Dot, Norm};
use math;
use std::f64::consts::PI;
use traits::{Bsdf};
use rand;
use rand::distributions::{Range, IndependentSample};
use {Color};
use color;
use utils::consts;


pub fn fresnel_schlick(normal: &Vector3f, light: &Vector3f, n1: Real, n2: Real) -> Real {
    let f0sqrt = (n1 - n2) / (n1 + n2);
    let f0 = f0sqrt * f0sqrt;
    let cos_theta = light.dot(&normal);
    f0 + (1.0 - f0) * (1.0 - cos_theta).powi(5)
}

pub fn fresnel3_schlick(normal: &Vector3f, light: &Vector3f, n1: &Vector3f, n2: &Vector3f) -> Vector3f {
    Vector3f::new(
        fresnel_schlick(normal, light, n1.x, n2.x),
        fresnel_schlick(normal, light, n1.y, n2.y),
        fresnel_schlick(normal, light, n1.z, n2.z))
}

pub fn fresnel_schlick_f0(cos_nl: Real, f0: Real) -> Real {
    f0 + (1.0 - f0) * (1.0 - cos_nl).powi(5)
}

pub fn fresnel3_schlick_f0(cos_nl: Real, f0: &Vector3f) -> Vector3f {
    Vector3f::new(
        fresnel_schlick_f0(cos_nl, f0.x),
        fresnel_schlick_f0(cos_nl, f0.y),
        fresnel_schlick_f0(cos_nl, f0.z))
}

pub fn calc_f0(n1: Real, n2: Real) -> Real {
    let f0sqrt = (n1 - n2) / (n1 + n2);
    f0sqrt * f0sqrt
}

pub fn calc3_f0(n1: &Vector3f, n2: &Vector3f) -> Vector3f {
    Vector3f::new(
        calc_f0(n1.x, n2.x),
        calc_f0(n1.y, n2.y),
        calc_f0(n1.z, n2.z),
    )
}

/// Microfacet distribution function
pub fn ggx_d(cos_nh: Real, alpha: Real) -> Real {
    let alpha2 = alpha * alpha;
    let cos_nh2 = cos_nh * cos_nh;
    let d  = alpha2 * cos_nh2 + (1.0 - cos_nh2);

    //ggx_chi(cos_nh) * 
    alpha2 / ((PI as Real) * d * d)
}

pub fn ggx_g_partial(cos_no: Real, cos_oh: Real, alpha: Real) -> Real {
    let alpha2 = alpha * alpha;
    let cos_no2 = cos_no * cos_no;
    let tan_nv2 = (1.0 - cos_no2) / cos_no2;

    //ggx_chi(cos_oh / cos_no) *
    2.0 / (1.0 + (1.0 + alpha2 * tan_nv2).sqrt())
}

/// Bidirectional shadowing-masking function
pub fn ggx_g(cos_nl: Real, cos_nv: Real, cos_lh: Real, cos_vh: Real, alpha: Real) -> Real {
    let g0 = ggx_g_partial(cos_nl, cos_lh, alpha);
    let g1 = ggx_g_partial(cos_nv, cos_vh, alpha);
    g1 * g0
}

pub fn ggx_chi(v: Real) -> Real {
    if v > 0.0 { 1.0 } else { 0.0 }
}

pub fn cooktorrance_reflection(
    cos_nl: Real,
    cos_nv: Real,
    cos_nh: Real,
    cos_lh: Real,
    cos_vh: Real,
    f0: &Vector3f,
    alpha: Real,) 
    -> Vector3f
{
    let g = ggx_g(cos_nl, cos_nv, cos_lh, cos_vh, alpha);
    let d = ggx_d(cos_nh, alpha);
    let f = fresnel3_schlick_f0(cos_nl, f0);
    f * (g * d / (4.0 * cos_nl * cos_nv))
}

#[inline]
pub fn cooktorrance_halfvec_refl(vec_in: &Vector3f, vec_out: &Vector3f) -> Vector3f {
    ((-vec_in) + vec_out).normalize()
}

#[inline]
pub fn cooktorrance_view_from_half_refl(light: &Vector3f, half: &Vector3f) -> Vector3f {
    math::reflect_vec(&light, &half)
}

pub fn sample_halfvec(normal: &Vector3f, alpha: Real) -> Vector3f {
    let mut rng = rand::thread_rng();
    let r01 = Range::new(0.0, 1.0 as Real);
    let u1 = r01.ind_sample(&mut rng);
    let u2 = r01.ind_sample(&mut rng);

    let theta = (alpha * u1.sqrt() / (1.0 - u1).sqrt()).atan();
    //let theta = ((1.0 - u1) / ((alpha * alpha - 1.0) * u1 + 1.0)).sqrt().acos();
    let phi = 2.0 * (PI as Real) * u2;

    let xs = theta.sin() * phi.cos();
    let ys = theta.cos();
    let zs = theta.sin() * phi.sin();

    math::transform_basis_y(normal, &Vector3f::new(xs, ys, zs))
}

pub fn sample_cooktorrance(
    normal: &Vector3f,
    vec_in: &Vector3f,
    f0: &Vector3f,
    alpha: Real) 
    -> (Vector3f, Vector3f, Real)
{
    let light = -vec_in;
    let half = sample_halfvec(&normal, alpha);
    let vec_out = cooktorrance_view_from_half_refl(&light, &half);
    let cos_nl = normal.dot(&light);
    let cos_nv = normal.dot(&vec_out);
    let cos_nh = normal.dot(&half);
    let cos_lh = half.dot(&light);
    let cos_vh = half.dot(&vec_out);
    if cos_vh > 0.0 &&
       cos_lh > 0.0 &&
       cos_nl > 0.0 &&
       cos_nv > 0.0
    {
        let fr = cooktorrance_reflection(cos_nl, cos_nv, cos_nh, cos_lh, cos_vh, f0, alpha);
        let pdf = sample_ct_pdf(cos_nh, cos_vh, alpha);
        (vec_out, fr, pdf)
    } else { 
        (*normal, Vector3f::new(0.0, 0.0, 0.0), 1.00000123456789)
    }
}

pub fn sample_ct_pdf(cos_nh: Real, cos_oh: Real, alpha: Real) -> Real {
    let d = ggx_d(cos_nh, alpha);
    d * cos_nh / (4.0 * cos_oh)
}

pub struct CookTorrance {
    albedo: Color,
    f0: Vector3f,
    alpha: Real,
}

impl CookTorrance {
    pub fn new(albedo: Color, f0: Vector3f, alpha: Real) -> Self {
        let alpha = if alpha < consts::REAL_EPSILON {
            consts::REAL_EPSILON.sqrt()
        } else {
            alpha
        };
        Self {
            albedo,
            f0,
            alpha,
        }
    }

}

impl Bsdf for CookTorrance {

    fn radiance(&self) -> Option<Color> {
        None
    }

    fn sample(
        &self, 
        surface_normal: &Vector3f, 
        in_dir: &Vector3f) 
        -> (Vector3f, Color, Real)
    {
        let (out_dir, fr, pdf) = sample_cooktorrance(surface_normal, &in_dir, &self.f0, self.alpha);
        let spek = Color::new(fr.x.abs() as f32, fr.y.abs() as f32, fr.z.abs() as f32);
        if pdf < consts::REAL_EPSILON {
            println!("cook-torrance: wrong pdf");
            (out_dir, color::BLACK, 1.00000123456789)
        } else {
            (out_dir, spek, pdf)
        }        
    }

    fn eval(
        &self, 
        surface_normal: &Vector3f,
        in_dir: &Vector3f,
        out_dir: &Vector3f)
        -> (Color, Real)
    {
        let half = cooktorrance_halfvec_refl(in_dir, out_dir);
        let light = -in_dir;
        let cos_nl = surface_normal.dot(&light);
        let cos_nv = surface_normal.dot(&out_dir);
        let cos_nh = surface_normal.dot(&half);
        let cos_lh = half.dot(&light);
        let cos_oh = half.dot(&out_dir);
        let fr = cooktorrance_reflection(cos_nl, cos_nv, cos_nh, cos_lh, cos_oh, &self.f0, self.alpha);        
        let pdf = sample_ct_pdf(cos_nh, cos_oh, self.alpha);
        (Color::new(fr.x.abs() as f32, fr.y.abs() as f32, fr.z.abs() as f32), pdf)
    }

}