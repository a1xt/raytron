use ::{Material, Color};
use math::{Vector3f, Point3f, Ray3f};
use math;

pub struct Diffuse {
    color: Color,
    emission: Option<Color>,
}

impl Diffuse {
    pub fn new(color: Color, emission: Option<Color>) -> Diffuse {
        Diffuse {
            color: color,
            emission: emission,
        }
    }
}

impl Material for Diffuse {

    fn emission(&self) -> Option<Color> {
        self.emission
    }

    fn reflectance(&self, _: &Vector3f, _: &Vector3f, _: &Vector3f) -> Color {
        self.color
    }

    fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f {
        Ray3f {
            origin: *surface_point,
            //dir: math::hs_uniform_sampling(surface_normal)
            dir: math::hs_cosine_sampling(surface_normal)
        }
    }

    fn brdf (&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color) {
        (
            Ray3f {
                origin: *surface_point,
                dir: math::hs_cosine_sampling(surface_normal),
            },
            self.color,
        )
    }
}
