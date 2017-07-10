use math::{self, Vector3f, Point3f, Ray3f, Real};
use num::Float;
use utils::consts;

#[derive(Copy, Clone, Debug)]
pub struct Aabb3 {
    pmin: Point3f,
    pmax: Point3f,
}

impl Aabb3 {
    pub fn new(pmin: Point3f, pmax: Point3f) -> Aabb3 {
        assert!(math::partial_le(&pmin, &pmax));
        Aabb3 { pmin, pmax }
    }

    pub fn mins(&self) -> &Point3f {
        &self.pmin
    }

    pub fn maxs(&self) -> &Point3f {
        &self.pmax
    }

    pub fn center(&self) -> Point3f {
        self.pmin + 0.5 * (self.pmax - self.pmin)
    }

    pub fn contains(&self, other: &Aabb3) -> bool {
        math::partial_le(&self.pmin, &other.pmin) && math::partial_ge(&self.pmax, &other.pmax)
    }

    pub fn intersects(&self, other: &Aabb3) -> bool {
        math::partial_le(&self.pmin, &other.pmax) && math::partial_ge(&self.pmax, &other.pmin)
    }

    pub fn contains_point(&self, point: &Point3f) -> bool {
        math::partial_le(&self.pmin, point) && math::partial_ge(&self.pmax, point)
    }

    pub fn volume(&self) -> Real {
        let mut vol = 1.0;
        for i in 0..3 {
            vol *= self.pmax[i] - self.pmin[i]
        }
        vol
    }

    pub fn surface_area(&self) -> Real {
        let a = self.pmax.x - self.pmin.x;
        let b = self.pmax.y - self.pmin.y;
        let c = self.pmax.z - self.pmin.z;
        2.0 * (a * b + b * c + a * c)
    }

    pub fn merge(&mut self, other: &Aabb3) {
        self.pmin = Point3f::new(
            self.pmin.x.min(other.pmin.x),
            self.pmin.y.min(other.pmin.y),
            self.pmin.z.min(other.pmin.z),
        );
        self.pmax = Point3f::new(
            self.pmax.x.max(other.pmax.x),
            self.pmax.y.max(other.pmax.y),
            self.pmax.z.max(other.pmax.z),
        )
    }
}

pub trait HasBounds {
    fn aabb(&self) -> Aabb3;
}


pub fn intersection_aabb(aabb: &Aabb3, ray: &Ray3f) -> Option<(Real, Real)> {
    let inv_dir: Vector3f = Vector3f::new(1.0, 1.0, 1.0) / ray.dir;

    let mut t_min = Real::max_value();
    let mut t_max = Real::min_value();

    for i in 0..3 {
        if ray.dir[i].abs() > consts::POSITION_EPSILON {
            let t1 = (aabb.pmin[i] - ray.origin[i]) * inv_dir[i];
            let t2 = (aabb.pmax[i] - ray.origin[i]) * inv_dir[i];

            t_min = t1.min(t2);
            t_max = t1.max(t2);
        }
    }

    if (t_min < 0.0 && t_max < 0.0) || t_min > t_max {
        None
    } else {
        Some((t_min, t_max))
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn contains() {
        let aabb0 = Aabb3::new(Point3f::new(-3.0, -3.0, -3.0), Point3f::new(3.0, 3.0, 3.0));
        let aabb1 = Aabb3::new(Point3f::new(-1.0, -1.0, -1.0), Point3f::new(1.0, 1.0, 1.0));
        let aabb2 = Aabb3::new(Point3f::new(-2.0, -2.0, -2.0), Point3f::new(4.0, 4.0, 4.0));

        assert!(aabb0.contains(&aabb1));
        assert!(!aabb0.contains(&aabb2));
        assert!(!aabb1.contains(&aabb0));
        assert!(!aabb1.contains(&aabb2));
        assert!(aabb2.contains(&aabb1));
        assert!(!aabb2.contains(&aabb0));
    }

    #[test]
    fn intersects() {
        let aabb0 = Aabb3::new(Point3f::new(-3.0, -3.0, -3.0), Point3f::new(3.0, 3.0, 3.0));
        let aabb1 = Aabb3::new(Point3f::new(-1.0, -1.0, -1.0), Point3f::new(1.0, 1.0, 1.0));
        let aabb2 = Aabb3::new(Point3f::new(-2.0, -2.0, -2.0), Point3f::new(4.0, 4.0, 4.0));
        let aabb3 = Aabb3::new(Point3f::new(3.5, 3.5, 3.5), Point3f::new(6.0, 6.0, 6.0));

        assert!(aabb0.intersects(&aabb2));
        assert!(aabb2.intersects(&aabb0));
        assert!(!aabb0.intersects(&aabb3));
        assert!(!aabb3.intersects(&aabb0));
        assert!(aabb2.intersects(&aabb3));
        assert!(aabb3.intersects(&aabb2));

        let aabb4 = Aabb3::new(Point3f::new(-1.0, 1.0, -1.0), Point3f::new(3.0, 5.0, 1.0));
        let aabb5 = Aabb3::new(Point3f::new(-3.0, -5.0, -1.0), Point3f::new(1.0, -1.0, 1.0));

        assert!(!aabb4.intersects(&aabb5));

    }
}
