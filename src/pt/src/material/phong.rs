use ::{Material, Color};
use math::{self, Vector3f, Point3f, Ray3f, Coord};

use color;
use std::f32::consts::{PI};
use rand;
use rand::{random, Closed01};
use rand::distributions::{Range, IndependentSample};
use math::{Cross, Norm, Dot};

pub struct Phong {
    color: Color,
    kd: f32,
    ks: f32,
    n: f32,
}

impl Phong {
    pub fn new (color: Color, kd: f32, ks: f32, n: f32) -> Phong {
        let mut s = ks;
        let mut d = kd;
        if ks + kd > 1.0 {
            s = s / (ks + kd);
            d = d / (ks + kd);
        }
        Phong {
            color: color,
            kd: d,
            ks: s,
            n: n,
        }
    }

    fn random_vector(&self, normal: &Vector3f) -> Vector3f {

        let mut rng = rand::thread_rng();
        let r01 = Range::new(0.0, 1.0 as Coord);
        let u1 = r01.ind_sample(&mut rng);
        let u2 = r01.ind_sample(&mut rng);

        let alpha = (1.0 - u1).powf(1.0 / (self.n as Coord + 1.0)).acos();
        let phi = 2.0 * (PI as Coord) * u2;

        let xs = alpha.sin() * phi.cos();
        let ys = alpha.cos();
        let zs = alpha.sin() * phi.sin();

        let y = normal.clone();
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
}

impl Material for Phong {
    fn emittance(&self) -> Option<Color> {
        None
    }

    // fn reflectance(&self, ray: &Vector3f, reflected_ray: &Vector3f, normal: &Vector3f) -> Color {
    //     // let Closed01(e) = rand::random<Closed01<f32>>();
    //     // if (e < self.kd) {

    //     // } else {

    //     // }
  
    //     let k = (self.n as Coord + 2.0) / (self.n as Coord + 1.0) * normal.dot(reflected_ray);
    //     color::mul_s(&self.color, k as f32)
    // }

    // fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f {
    //     Ray3f {
    //         origin: *surface_point,
    //         dir: self.random_vector(surface_normal),
    //     }
    // }

    // fn brdf (&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color) {
    //     let Closed01(e) = rand::random::<Closed01<f32>>();

    //     if e < self.kd {
    //         (
    //             Ray3f {
    //                 origin: *surface_point,
    //                 dir: math::hs_cosine_sampling(surface_normal),
    //             },
    //             self.color
    //         )
            
    //     } else if e < self.kd + self.ks {
    //         let cos_theta = surface_normal.dot(&(-ray_dir)).abs();
    //         let ir = (surface_normal * 2.0 + ((-ray_dir) / cos_theta) * (-1.0)).normalize();
    //         let r = Ray3f {
    //             origin: *surface_point,
    //             dir: self.random_vector(&ir),
    //         };
     
    //         let cos_theta = surface_normal.dot(&r.dir).abs();
    //         let k = ((self.n as Coord + 2.0) / (self.n as Coord + 1.0)) * cos_theta;// * (1.0 / ps);
    //         let c = color::mul_s(&self.color, k as f32);
    //         //let c = color::mul_s(&color::WHITE, k);

    //         (r, c)

    //     } else {
    //         (Ray3f{origin: *surface_point, dir: math::zero()}, color::BLACK)
    //     } 

    // }

    fn eval_bsdf(
        &self, 
        surface_normal: &Vector3f,
        in_dir: &Vector3f,
        out_dir: &Vector3f,        
    ) -> (Color, Coord)
    {
        let Closed01(e) = rand::random::<Closed01<f32>>();
        if e < self.kd {
            let pdf = out_dir.dot(&surface_normal) / PI as Coord;

            (color::mul_s(&self.color, 1.0 / PI), pdf)

        } else if e < self.kd + self.ks {
            let n = self.n as Coord;

            let cos_theta_in = surface_normal.dot(&(-in_dir));
            let in_dir_refl = (surface_normal * 2.0 + ((-in_dir) / cos_theta_in) * (-1.0)).normalize();
            
            let cos_alpha = in_dir_refl.dot(&out_dir);
            if cos_alpha > 0.0 {
                let pdf = (n + 1.0) * cos_alpha.powf(n) / (2.0 * PI as Coord);

                let f = ((n + 2.0) / (2.0 * PI as Coord)) * cos_alpha.powf(n);
                let fr = color::mul_s(&self.color, f as f32);

                (fr, pdf)
            } else {
                (color::BLACK, 1.0)
            }

        } else {
            (color::BLACK, 1.0)
        }
    }      


    fn sample_bsdf(&self, surface_normal: &Vector3f, in_dir: &Vector3f) -> (Vector3f, Color, Coord) {
        let Closed01(e) = rand::random::<Closed01<f32>>();
        if e < self.kd {
            let out_dir = math::hs_cosine_sampling(&surface_normal);
            let pdf = out_dir.dot(&surface_normal);

            (out_dir, self.color, pdf)

        } else if e < self.kd + self.ks {
            let n = self.n as Coord;

            let cos_theta_in = surface_normal.dot(&(-in_dir));
            let in_dir_refl = (surface_normal * 2.0 + ((-in_dir) / cos_theta_in) * (-1.0)).normalize();
            let out_dir = self.random_vector(&in_dir_refl);
            
            let f = (n + 2.0) / (n + 1.0);
            let fr = color::mul_s(&self.color, f as f32);

            (out_dir, fr, 1.0)

        } else {
            (math::zero(), color::BLACK, 1.0)
        }
    }
}