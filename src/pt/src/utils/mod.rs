pub mod consts;

use std::f32::consts::PI;

pub fn to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}
