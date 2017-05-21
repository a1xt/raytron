use image::{Rgb, Rgba};
use image;
use std::f32;

pub type Rgba32f = Rgba<f32>;
pub type Rgb32f = Rgb<f32>;
pub type Rgba8 = Rgba<u8>;
pub type Rgb8 = Rgb<u8>;
pub type Color = Rgba32f;

pub type Image = image::ImageBuffer<Color, Vec<f32>>;
pub type Texture = image::ImageBuffer<Color, Vec<u8>>:

pub const BLACK: Color = Color {data: [0.0, 0.0, 0.0, 1.0]};
pub const WHITE: Color = Color {data: [1.0, 1.0, 1.0, 1.0]};
pub const RED: Color = Color {data: [1.0, 0.0, 0.0, 1.0]};
pub const GREEN: Color = Color {data: [0.0, 1.0, 0.0, 1.0]};
pub const BLUE: Color = Color {data: [0.0, 0.0, 1.0, 1.0]};


pub fn clamp_alpha (color: &Color) -> Color {
    let mut c = *color;
    if c[3] > 1.0 {
        c[3] = 1.0
    }
    c
}

pub fn clamp_rgba(c: &Color) -> Color {
    let clamp = |a| { if a > 1.0 { 1.0 } else if a < 0.0 { 0.0 } else { a }};
    Color { data: [
        clamp(c[0]),
        clamp(c[1]),
        clamp(c[2]),
        1.0,
    ]}
}

pub fn round_rgba (c: &Color) -> Color {
    if c[0] > 1.0 || c[1] > 1.0 || c[2] > 1.0 {
        let l = 1.0 / f32::max(f32::max(c[0], c[1]), f32::max(c[0], c[2]));
        Color { data: [
            c[0] * l,
            c[1] * l,
            c[2] * l,
            1.0,
        ]}
    } else {
        *c
    }
}

pub fn sum (c0: &Color, c1: &Color) -> Color {
    let c = Color { data: [
        c0[0] + c1[0],
        c0[1] + c1[1],
        c0[2] + c1[2],
        1.0,
    ]};
    c
}

pub fn mul_s(color: &Color, s: f32) -> Color {
    let c = Color { data: [
        color[0] * s,
        color[1] * s,
        color[2] * s,
        1.0,
    ]};
    c
}

pub fn mul_v(c0: &Color, c1: &Color) -> Color {
    let c = Color { data: [
        c0[0] * c1[0],
        c0[1] * c1[1],
        c0[2] * c1[2],
        1.0,
    ]};
    c
}

pub fn rgb_to_illumination (c: &Color) -> f32 {
    c[0] + c[1] + c[2] / 3.0
}
