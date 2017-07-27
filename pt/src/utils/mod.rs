pub mod consts;

use color::Rgb;
use math::Real;

pub fn clamp<T: Copy + PartialOrd>(val: T, left_bound: T, right_bound: T) -> T {
    if val < left_bound {
        left_bound
    } else if val > right_bound {
        right_bound
    } else {
        val
    }

}

pub fn normal_dx_to_ogl(n: &Rgb<Real>) -> Rgb<Real> {
    let n_dx = *n * 2.0 - Rgb::<Real>::from(1.0);
    let n_gl = Rgb::new(n_dx.r, -n_dx.g, n_dx.b);
    let res = (n_gl + Rgb::<Real>::from(1.0)) * 0.5;
    assert!(res.g >= 0.0 && res.g <= 1.0);
    res
}
