use Color;
use color::Rgb;
use math;
use math::{Dot, Norm, Real, Vector3f};
use rand;
use rand::Closed01;
use rand::distributions::{IndependentSample, Range};
use std::f64::consts::PI;
use traits::Bsdf;
use utils::consts;

/// Microfacet distribution function
pub fn ggx_d(cos_nh: Real, alpha: Real) -> Real {
    let alpha2 = alpha * alpha;
    let cos_nh2 = cos_nh * cos_nh;
    let d = alpha2 * cos_nh2 + (1.0 - cos_nh2);

    //ggx_chi(cos_nh) *
    alpha2 / ((PI as Real) * d * d)
}

pub fn ggx_g_partial(cos_no: Real, _: Real, alpha: Real) -> Real {
    let alpha2 = alpha * alpha;
    let cos_no2 = cos_no * cos_no;
    let tan_no2 = (1.0 - cos_no2) / cos_no2;

    //ggx_chi(cos_oh / cos_no) *
    2.0 / (1.0 + (1.0 + alpha2 * tan_no2).sqrt())
}

/// Bidirectional shadowing-masking function
pub fn ggx_g(cos_nl: Real, cos_nv: Real, cos_lh: Real, cos_vh: Real, alpha: Real) -> Real {
    let g0 = ggx_g_partial(cos_nl, cos_lh, alpha);
    let g1 = ggx_g_partial(cos_nv, cos_vh, alpha);
    g1 * g0
}

pub fn ggx_chi(v: Real) -> Real {
    if v > 0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn eval_reflectance(
    cos_nl: Real,
    cos_nv: Real,
    cos_nh: Real,
    cos_lh: Real,
    cos_vh: Real,
    f0: &Rgb<Real>,
    alpha: Real,
) -> (Rgb<Real>, Real) {
    let f = math::fresnel3_schlick_f0(cos_lh, f0);
    let g = ggx_g(cos_nl, cos_nv, cos_lh, cos_vh, alpha);
    let d = ggx_d(cos_nh, alpha);
    let fr = f * (g * d / (4.0 * cos_nl * cos_nv));
    let pdf = pdf_refl(cos_nh, cos_vh, alpha);
    (fr, pdf)
}

pub fn eval(
    normal: &Vector3f,
    vec_in: &Vector3f,
    vec_out: &Vector3f,
    f0: &Rgb<Real>,
    albedo: &Rgb<Real>,
    alpha: Real,
) -> (Rgb<Real>, Real) {
    let light = -vec_in;
    let half = calc_halfvec_refl(vec_in, vec_out);
    let cos_nl = normal.dot(&light);
    let cos_nv = normal.dot(vec_out);
    let cos_nh = normal.dot(&half);
    let cos_lh = half.dot(&light);
    let cos_oh = half.dot(vec_out);
    let f = math::fresnel3_schlick_f0(cos_lh, f0);

    let (spec, spec_pdf) = eval_reflectance(cos_nl, cos_nv, cos_nh, cos_lh, cos_oh, f0, alpha);
    let diff_f = Rgb::<Real>::from(1.0) - f;
    let diff = diff_f * (1.0 / PI) * (*albedo);
    let diff_pdf = cos_nv / PI;
    (spec + diff, 0.5 * (spec_pdf + diff_pdf))
}

#[inline]
pub fn calc_halfvec_refl(vec_in: &Vector3f, vec_out: &Vector3f) -> Vector3f {
    ((-vec_in) + vec_out).normalize()
}

#[inline]
pub fn calc_view(light: &Vector3f, half: &Vector3f) -> Vector3f {
    math::reflect_vec(light, half)
}

pub fn sample_halfvec(normal: &Vector3f, alpha: Real) -> Vector3f {
    let mut rng = rand::thread_rng();
    let r01 = Range::new(0.0, 1.0 as Real);
    let u1 = r01.ind_sample(&mut rng);
    let u2 = r01.ind_sample(&mut rng);

    //let theta = (alpha * u1.sqrt() / (1.0 - u1).sqrt()).atan();
    let theta = ((1.0 - u1) / ((alpha * alpha - 1.0) * u1 + 1.0))
        .sqrt()
        .acos();
    let phi = 2.0 * (PI as Real) * u2;

    let xs = theta.sin() * phi.cos();
    let ys = theta.cos();
    let zs = theta.sin() * phi.sin();

    math::transform_basis_y(normal, &Vector3f::new(xs, ys, zs))
}

pub fn sample(
    normal: &Vector3f,
    vec_in: &Vector3f,
    f0: &Rgb<Real>,
    albedo: &Rgb<Real>,
    alpha: Real,
) -> (Vector3f, Rgb<Real>, Real) {

    let light = -vec_in;
    let vec_out = loop {

        let half = sample_halfvec(normal, alpha);
        let vec_out = calc_view(&light, &half);
        let cos_vh = half.dot(&vec_out);
        let cos_nv = normal.dot(&vec_out);

        if cos_vh > 0.0 && cos_nv > 0.0 {
            break vec_out;
        }
    };

    let ks = 0.5;
    let Closed01(e) = rand::random::<Closed01<Real>>();

    let vec_out = if e <= ks {
        vec_out
    } else {
        math::hs_cosine_sampling(normal)
    };

    let (fr, pdf) = eval(normal, vec_in, &vec_out, f0, albedo, alpha);
    (vec_out, fr, pdf)

}

pub fn pdf_refl(cos_nh: Real, cos_oh: Real, alpha: Real) -> Real {
    let d = ggx_d(cos_nh, alpha);
    d * cos_nh / (4.0 * cos_oh)
}

pub struct CookTorrance {
    albedo: Rgb<Real>,
    f0: Rgb<Real>,
    alpha: Real,
}

impl CookTorrance {
    pub fn new<Ca, Cf>(albedo: Ca, f0: Cf, alpha: Real) -> Self
    where
        Rgb<Real>: From<Ca> + From<Cf>,
    {
        let alpha = if alpha < consts::REAL_EPSILON {
            consts::REAL_EPSILON.sqrt()
        } else {
            alpha
        };

        Self {
            albedo: albedo.into(),
            f0: f0.into(),
            alpha,
        }
    }
}

impl Bsdf for CookTorrance {
    fn radiance(&self) -> Option<Color> {
        None
    }

    fn sample(&self, surface_normal: &Vector3f, in_dir: &Vector3f) -> (Vector3f, Color, Real) {
        let (out_dir, fr, pdf) = sample(surface_normal, in_dir, &self.f0, &self.albedo, self.alpha);
        let spek = Color::new(fr.r.abs() as f32, fr.g.abs() as f32, fr.b.abs() as f32);
        (out_dir, spek, pdf)

    }

    fn eval(
        &self,
        surface_normal: &Vector3f,
        in_dir: &Vector3f,
        out_dir: &Vector3f,
    ) -> (Color, Real) {

        let (fr, pdf) = eval(
            surface_normal,
            in_dir,
            out_dir,
            &self.f0,
            &self.albedo,
            self.alpha,
        );

        (
            Color::new(fr.r.abs() as f32, fr.g.abs() as f32, fr.b.abs() as f32),
            pdf,
        )
    }
}
