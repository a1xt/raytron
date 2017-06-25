use math::{Vector3f, Real, Dot, Norm};
use math;
use std::f64::consts::PI;
use traits::{Bsdf};
use rand;
use rand::distributions::{Range, IndependentSample};
use {Color};
use color;
use utils::consts;

/// Microfacet distribution function
pub fn ggx_d(normal: &Vector3f, half: &Vector3f, alpha: Real) -> Real {
    let cos_theta = half.dot(&normal);
    let alpha2 = alpha * alpha;
    let cos_theta2 = cos_theta * cos_theta;
    let d  = alpha2 * cos_theta2 + (1.0 - cos_theta2);

    //ggx_chi(cos_theta) * 
    alpha2 / ((PI as Real) * d * d)
}

pub fn ggx_g_partial(normal: &Vector3f, vec_out: &Vector3f, half: &Vector3f, alpha: Real) -> Real {
    let cos_vn = vec_out.dot(&normal);
    let cos_vm = vec_out.dot(&half);
    let alpha2 = alpha * alpha;
    let cos_vn2 = cos_vn * cos_vn;
    let tan_vn2 = (1.0 - cos_vn2) / cos_vn2;

    //ggx_chi(cos_vm / cos_vn) *
    2.0 / (1.0 + (1.0 + alpha2 * tan_vn2).sqrt())
}

/// Bidirectional shadowing-masking function
pub fn ggx_g(normal: &Vector3f, light: &Vector3f, view: &Vector3f, half: &Vector3f, alpha: Real) -> Real {
    let g0 = ggx_g_partial(normal, view, half, alpha);
    let g1 = ggx_g_partial(normal, light, half, alpha);
    g1 * g0
}

pub fn ggx_chi(v: Real) -> Real {
    if v > 0.0 { 1.0 } else { 0.0 }
}

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
        fresnel_schlick(normal, light, n1.z, n2.z),
    )
}

pub fn cooktorrance_reflection(
    normal: &Vector3f,
    vec_in: &Vector3f,
    vec_out: &Vector3f,
    ior1: &Vector3f,
    ior2: &Vector3f,
    alpha: Real,) 
    -> Vector3f
{
    let light = -vec_in;
    let half = cooktorrance_halfvec_refl(vec_in, vec_out);
    let g = ggx_g(normal, &light, vec_out, &half, alpha);
    let d = ggx_d(normal, &half, alpha);
    let f = fresnel3_schlick(normal, &light, ior1, ior2);
    f * (g * d / (4.0 * normal.dot(&light) * normal.dot(&vec_out)))
}

#[inline]
pub fn cooktorrance_halfvec_refl(vec_in: &Vector3f, vec_out: &Vector3f) -> Vector3f {
    ((-vec_in) + vec_out).normalize()
}

#[inline]
pub fn cooktorrance_refl_air(
    normal: &Vector3f,
    vec_in: &Vector3f,
    vec_out: &Vector3f,
    ior: &Vector3f,
    alpha: Real,)
    -> Vector3f
{
    let air_ior = Vector3f::new(1.0, 1.0, 1.0);
    cooktorrance_reflection(normal, vec_in, vec_out, &air_ior, ior, alpha)
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
    ior: &Vector3f,
    alpha: Real) 
    -> (Vector3f, Vector3f, Real)
{
    let light = -vec_in;
    let half = sample_halfvec(&normal, alpha);
    let vec_out = math::reflect_vec(&light, &half);
    if half.dot(&light) > 0.0 &&
       half.dot(&vec_out) > 0.0 &&
       normal.dot(&light) > 0.0 &&
       normal.dot(&vec_out) > 0.0
    {
        let fr = cooktorrance_refl_air(normal, vec_in, &vec_out, ior, alpha);
        let pdf = sample_ct_pdf(normal, &vec_out, &half, alpha);
        (vec_out, fr, pdf)
    } else { 
        (*normal, Vector3f::new(0.0, 0.0, 0.0), 1.00000111010101)
    }
}

pub fn sample_ct_pdf(normal: &Vector3f, vec_out: &Vector3f, half: &Vector3f, alpha: Real) -> Real {
    let d = ggx_d(half, normal, alpha);
    let nh = half.dot(normal);
    let ho = half.dot(vec_out);
    d * nh / (4.0 * ho)
}

pub struct CookTorrance {
    albedo: Color,
    ior: Vector3f,
    alpha: Real,
}

impl CookTorrance {
    pub fn new(albedo: Color, ior: Vector3f, alpha: Real) -> Self {
        let alpha = if alpha < consts::REAL_EPSILON {
            consts::REAL_EPSILON.sqrt()
        } else {
            alpha
        };
        Self {
            albedo,
            ior,
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
        let (out_dir, fr, pdf) = sample_cooktorrance(surface_normal, &in_dir, &self.ior, self.alpha);
        let spek = Color::new(fr.x.abs() as f32, fr.y.abs() as f32, fr.z.abs() as f32);
        if pdf < consts::REAL_EPSILON {
            println!("cook-torrance: wrong pdf");
            (out_dir, color::BLACK, 1.00000111000111000111)
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
        let fr = cooktorrance_refl_air(surface_normal, in_dir, out_dir, &self.ior, self.alpha);
        let half = cooktorrance_halfvec_refl(in_dir, out_dir);
        let pdf = sample_ct_pdf(surface_normal, out_dir, &half, self.alpha);
        (Color::new(fr.x as f32, fr.y as f32, fr.z as f32), pdf)
    }

}