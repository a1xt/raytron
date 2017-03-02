pub extern crate nalgebra as na;
pub use self::na::*;


pub type Coord = f64;

pub type Ray3f = Ray3<Coord>;
pub type Matrix4f = Matrix4<Coord>;
pub type Vector3f = Vector3<Coord>;
pub type Vector4f = Vector4<Coord>;
pub type Point3f = Point3<Coord>;
pub type Point4f = Point4<Coord>;

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

pub fn intersection_with_sphere(sphere_pos: &Point3f, sphere_radius: Coord, ray: &Ray3f) -> Option<Coord> {
    debug_assert!(ray.dir.norm().abs_sub(1.0) <= 1.0e-6);
    use std;

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
pub fn barycetric_coords<T: BaseFloat>(v0: &Point3<T>, v1: &Point3<T>, v2: &Point3<T>, ray: &Ray3<T>) -> (T, (T, T)) {
    (zero(), (zero(), zero()))
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
    let r01 = Range::new(0.0, 1.0 as Coord);
    let u1 = r01.ind_sample(&mut rng);
    let u2 = r01.ind_sample(&mut rng);

    let theta = (1.0 - u1).sqrt().acos();
    let phi = 2.0 * (PI as Coord) * u2;

    let xs = theta.sin() * phi.cos();
    let ys = theta.cos();
    let zs = theta.sin() * phi.sin();

    let y = n.clone();
    let mut h = y.clone();

    if h.x.abs() <= h.y.abs() && h.x.abs() <= h.z.abs() {
        h.x = 1.0;
    } else if h.y.abs() <= h.x.abs() && h.y.abs() <= h.z.abs() {
        h.y = 1.0;
    } else {
        h.z = 1.0;
    }

    let x = h.cross(&y).normalize();
    let z = x.cross(&y).normalize();

    let dir = x * xs + y * ys + z * zs;

    dir.normalize()

}  


pub fn sph_uniform_sampling() -> Vector3f {
    let mut vec = zero();
    loop {
        let Closed01(mut x) = random::<Closed01<Coord>>();
        let Closed01(mut y) = random::<Closed01<Coord>>();
        let Closed01(mut z) = random::<Closed01<Coord>>();
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
