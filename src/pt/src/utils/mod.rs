pub mod consts;

use std::f32::consts::PI;
use traits::{Surface};
use {SurfacePoint};
use math::{self, Point3f, Coord};

pub fn to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// @return (point at surface, pdf)
pub fn sample_surfaces<'s, T: Iterator<Item = &'s Surface>>(mut surfaces: T, view_point: &Point3f) -> Option<(SurfacePoint<'s>, Coord)> {
    if let Some(s) = surfaces.next() {
        let (sp, pdf) = s.sample_surface(view_point);
        Some((sp, pdf))
    } else {
        None
    }

}