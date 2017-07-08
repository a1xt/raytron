pub extern crate nalgebra as na;
pub use self::na::*;


pub type Real = f64;

pub type Ray3f = Ray3<Real>;
pub type Matrix4f = Matrix4<Real>;
pub type Vector3f = Vector3<Real>;
pub type Vector4f = Vector4<Real>;
pub type Point3f = Point3<Real>;
pub type Point4f = Point4<Real>;

use std::f32::EPSILON;
pub const FLOAT_EPSILON: Real = EPSILON as Real;
use color::Rgb;

#[derive(Copy, Clone, Debug)]
pub struct Ray3<F> where F: Copy + Clone {
    pub origin: Point3<F>,
    pub dir: Vector3<F>,
}

impl<F> Ray3<F> where F: Copy + Clone {
    pub fn new(origin: &Point3<F>, dir: &Vector3<F>) -> Ray3<F> {
        Ray3 {
            origin: *origin,
            dir: *dir,
        }
    }
}

pub fn intersection_sphere(sphere_pos: &Point3f, sphere_radius: Real, ray: &Ray3f) -> Option<Real> {
    debug_assert!((ray.dir.norm() - 1.0).abs() <= 1.0e-6);

    let l = ray.origin - *sphere_pos;
    let b = dot(&l, &ray.dir);
    let c = dot(&l, &l) - sphere_radius * sphere_radius;
    let d2 = b * b - c;
    if d2 >= 0.0 {
        //let d = d2.sqrt();
        let t_min = -b - (b * b - c).sqrt();
        let t_max = -b + (b * b - c).sqrt();
        // let t_min = -b - d;
        // let t_max = -b + d;
        //let t1 = -b - d;
        //let t2 = -b + d;
        //let (t_min, t_max) = (T::min(t1, t2), T::max(t1, t2));

        if t_min >= 0.0 {
            return Some(t_min);
        } 
        else if t_max > 0.0 {
            return Some(t_max);
        }
    }
    None
}

/// return (t, (u, v))
pub fn intersection_triangle(v0: &Point3f, v1: &Point3f, v2: &Point3f, ray: &Ray3f, culling: bool) -> Option<(Real, (Real, Real))> {

    let e1 = *v2 - *v0;
    let e2 = *v1 - *v0;    
    let p = ray.dir.cross(&e2);
    let det = p.dot(&e1);

    let d = if culling { det } else { det.abs() };
    if d < FLOAT_EPSILON {
        return None;
    }

    let t0 = ray.origin - *v0;
    let q = t0.cross(&e1);
    let det_inv = 1.0 / det;

    let t = det_inv * q.dot(&e2);
    let u = det_inv * p.dot(&t0);
    let v = det_inv * q.dot(&ray.dir);

    if t < 0.0 || u < 0.0 || v < 0.0 || u + v > 1.0 {
        return None;
    }

    Some((t, (u, v)))
}

#[inline]
pub fn triangle_area(v0: &Point3f, v1: &Point3f, v2: &Point3f) -> Real {
    let b = *v1 - *v0;
    let c = *v2 - *v0;
    0.5 * b.cross(&c).norm()
}

#[inline]
pub fn triangle_normal(v0: &Point3f, v1: &Point3f, v2: &Point3f) -> Vector3f {
    let a = *v1 - *v0;
    let b = *v2 - *v0;
    b.cross(&a).normalize()
}

use rand;
use rand::{random, Closed01};
use rand::distributions::{Range, IndependentSample};


pub fn hs_uniform_sampling(hemisphere_normal: &Vector3f) -> Vector3f {
    let vec = sph_uniform_sampling();

    if vec.dot(hemisphere_normal) > 0.0 {
        vec
    } else {
        vec * (-1.0)
    }

}

pub fn hs_cosine_sampling(n: &Vector3f) -> Vector3f {
    //use std::f32::{cos, sin};
    use std::f64::consts::{PI};

    let mut rng = rand::thread_rng();
    let r01 = Range::new(0.0, 1.0 as Real);
    let u1 = r01.ind_sample(&mut rng);
    let u2 = r01.ind_sample(&mut rng);

    let theta = (1.0 - u1).sqrt().acos();
    let phi = 2.0 * (PI as Real) * u2;

    let xs = theta.sin() * phi.cos();
    let ys = theta.cos();
    let zs = theta.sin() * phi.sin();

    // let y = n.clone();
    // let mut h = y.clone();

    // if h.x.abs() <= h.y.abs() && h.x.abs() <= h.z.abs() {
    //     h.x = 1.0;
    // } else if h.y.abs() <= h.x.abs() && h.y.abs() <= h.z.abs() {
    //     h.y = 1.0;
    // } else {
    //     h.z = 1.0;
    // }

    // let x = h.cross(&y).normalize();
    // let z = x.cross(&y).normalize();

    // let dir = x * xs + y * ys + z * zs;

    // dir.normalize()

    transform_basis_y(n, &Vector3f::new(xs, ys, zs))

}

pub fn transform_basis_y(up: &Vector3f, vec: &Vector3f) -> Vector3f {
    let y = *up;
    let mut h = y;

    if h.x.abs() <= h.y.abs() && h.x.abs() <= h.z.abs() {
        h.x = 1.0;
    } else if h.y.abs() <= h.x.abs() && h.y.abs() <= h.z.abs() {
        h.y = 1.0;
    } else {
        h.z = 1.0;
    }

    let x = h.cross(&y).normalize();
    let z = x.cross(&y).normalize();

    let dir = x * vec.x + y * vec.y + z * vec.z;

    dir.normalize()
}

pub fn reflect_vec(vec: &Vector3f, base: &Vector3f) -> Vector3f {
    let cos_theta = vec.dot(base);
    let h = base * 2.0 * cos_theta;
    h - vec
}

pub fn calc_tangent(
    (e1, du1, dv1): (&Vector3f, Real, Real), 
    (e2, du2, dv2): (&Vector3f, Real, Real)) 
    -> (Vector3f, Vector3f)
{
    let d = 1.0 / (du1 * dv2 - du2 * dv1);
    let t = d * Vector3f::new(
        dv2 * e1.x - dv1 * e2.x,
        dv2 * e1.y - dv1 * e2.y,
        dv2 * e1.z - dv1 * e2.z);
    let b = d * Vector3f::new(
        du1 * e2.x - du2 * e1.x,
        du1 * e2.y - du2 * e1.y,
        du1 * e2.z - du2 * e1.z);

    (t.normalize(), b.normalize())
}


pub fn sph_uniform_sampling() -> Vector3f {
    let vec;
    loop {
        let Closed01(mut x) = random::<Closed01<Real>>();
        let Closed01(mut y) = random::<Closed01<Real>>();
        let Closed01(mut z) = random::<Closed01<Real>>();
        x -= 0.5;
        y -= 0.5;
        z -= 0.5;

        if x * x + y * y + z * z < 0.25 {
            vec = Vector3::new(x, y, z).normalize();
            break;
        }
    }
    vec
}


pub fn fresnel_schlick(normal: &Vector3f, light: &Vector3f, n1: Real, n2: Real) -> Real {
    let f0sqrt = (n1 - n2) / (n1 + n2);
    let f0 = f0sqrt * f0sqrt;
    let cos_theta = light.dot(normal);
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

pub fn fresnel3_schlick_f0(cos_nl: Real, f0: &Rgb<Real>) -> Rgb<Real> {
    Rgb::new(
        fresnel_schlick_f0(cos_nl, f0.r),
        fresnel_schlick_f0(cos_nl, f0.g),
        fresnel_schlick_f0(cos_nl, f0.b))
}

pub fn calc_f0(n1: Real, n2: Real) -> Real {
    let f0sqrt = (n1 - n2) / (n1 + n2);
    f0sqrt * f0sqrt
}

pub fn calc3_f0(n1: &Rgb<Real>, n2: &Rgb<Real>) -> Rgb<Real> {
    Rgb::new(
        calc_f0(n1.r, n2.r),
        calc_f0(n1.g, n2.g),
        calc_f0(n1.b, n2.b),
    )
}