use core::array::FixedSizeArray;

use texture;
use num::{One, Zero, FromPrimitive, ToPrimitive, Bounded};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::u8;
use utils::{clamp};


pub type Color = Rgb;
pub type Image = texture::Texture<f32, [f32; 4], Rgb>;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);


pub trait ColorChannel: Copy + PartialEq + PartialOrd + One + Zero + ColorBounds + FromPrimitive + ToPrimitive + Bounded + Default {}
impl ColorChannel for u8 {}
impl ColorChannel for f32 {}

pub trait ColorBounds {
    const MIN_CHVAL: Self;
    const MAX_CHVAL: Self;
}

impl ColorBounds for u8 {
    const MIN_CHVAL: u8 = 0;
    const MAX_CHVAL: u8 = u8::MAX;
}
impl ColorBounds for f32 {
    const MIN_CHVAL: f32 = 0.0;
    const MAX_CHVAL: f32 = 1.0;
}

pub trait ColorClamp {
    fn clamp(self) -> Self;
}

pub trait ColorBlend<T: ColorChannel> {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self;
}

pub trait Pixel<T: ColorChannel, R: FixedSizeArray<T>> : Copy + ColorBlend<T> + From<R> + Into<R> { } 

impl<C, T, R> Pixel<T, R> for C
    where T: ColorChannel,
          R: FixedSizeArray<T>,
          C: Copy + ColorBlend<T> + Into<R> + From<R> {}

          impl<T: ColorChannel> ChannelCast<T> for T {
    #[inline]
    fn cast_from(other: T) -> T {
        other
    }
    
    #[inline]
    fn cast_into(self) -> T {
        self
    }
}

pub trait ChannelCast<T: ColorChannel>: ColorChannel {
    fn cast_from(other: T) -> Self;
    fn cast_into(self) -> T;
}

impl ChannelCast<f32> for u8 {
    #[inline]
    fn cast_from(other: f32) -> u8 {
        (clamp(other, f32::MIN_CHVAL, f32::MAX_CHVAL) * (u8::MAX_CHVAL as f32)) as u8
    }
    
    #[inline]
    fn cast_into(self) -> f32 {
        (self as f32) / (u8::MAX_CHVAL as f32)
    }
}

impl ChannelCast<u8> for f32 {
    #[inline]
    fn cast_from(other: u8) -> f32 {
        other.cast_into()
    }
    
    #[inline]
    fn cast_into(self) -> u8 {
        u8::cast_from(self)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Rgb<T = f32> where T: ColorChannel {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: ColorChannel> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Rgb {r, g, b}
    }
}

impl ColorBlend<f32> for Rgb<f32> {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self {
        c0 * w0 + c1 * w1
    }
}

impl ColorBlend<u8> for Rgb<u8> {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self {
        let r = w0 * (c0.r as f32) + w1 * (c1.r as f32);
        let g = w0 * (c0.g as f32) + w1 * (c1.g as f32);
        let b = w0 * (c0.b as f32) + w1 * (c1.b as f32);

        Self::new(
            clamp(r, 0.0, u8::MAX as f32) as u8,
            clamp(g, 0.0, u8::MAX as f32) as u8,
            clamp(b, 0.0, u8::MAX as f32) as u8,
        )
    }
}

impl<T> ColorClamp for Rgb<T> where T: ColorChannel {
    fn clamp(self) -> Self {
        Self::new(
            clamp(self.r, T::MIN_CHVAL, T::MAX_CHVAL),
            clamp(self.r, T::MIN_CHVAL, T::MAX_CHVAL),
            clamp(self.r, T::MIN_CHVAL, T::MAX_CHVAL),
        )
    }
}

impl<T> From<[T; 3]> for Rgb<T> where T: ColorChannel {
    fn from(v: [T; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}

impl<T> From<[T; 4]> for Rgb<T> where T: ColorChannel {
    fn from(v: [T; 4]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}

impl<T> Into<[T; 4]> for Rgb<T> where T: ColorChannel {
    fn into(self) -> [T; 4] {
        [self.r, self.g, self.b, T::MAX_CHVAL]
    }
>>>>>>> Added Texture type.
}

impl<T> Into<[T; 3]> for Rgb<T> where T: ColorChannel {
    fn into(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<Rgb<u8>> for Rgb<f32> {
    fn from(rgb: Rgb<u8>) -> Self {
        Self::new(
            rgb.r.cast_into(),
            rgb.g.cast_into(),
            rgb.b.cast_into())
    }
}

impl From<Rgb<f32>> for Rgb<u8> {
    fn from(rgb: Rgb<f32>) -> Self {
        Self::new(
            rgb.r.cast_into(),
            rgb.g.cast_into(),
            rgb.b.cast_into())
    }
}

// waiting for 'complementary traits' feature...
// impl<T, U> From<Rgb<T>> for Rgb<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
//     fn from(rgb: Rgb<T>) -> Self {
//         Self::new(
//             rgb.r.cast_into(),
//             rgb.g.cast_into(),
//             rgb.b.cast_into())
//     }
// }

impl<T, U> From<Rgba<T>> for Rgb<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
    fn from(rgba: Rgba<T>) -> Self {
        Self::new(
            rgba.r.cast_into(),
            rgba.g.cast_into(),
            rgba.b.cast_into())
    }
}

impl<T, U> From<Luma<T>> for Rgb<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
    fn from(luma: Luma<T>) -> Self {
        Self::new(
            luma.luma.cast_into(),
            luma.luma.cast_into(),
            luma.luma.cast_into())
    }
}

impl<T, U> From<LumaA<T>> for Rgb<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
    fn from(luma: LumaA<T>) -> Self {
        Self::new(
            luma.luma.cast_into(),
            luma.luma.cast_into(),
            luma.luma.cast_into())
    }
}

impl<T> From<T> for Rgb<T> where T: ColorChannel {
    fn from(val: T) -> Self {
        Self::new(val, val, val)
    }
}

impl<T> AsRef<[T; 3]> for Rgb<T> where T: ColorChannel {
    fn as_ref(&self) -> &[T; 3] {
        unsafe {
            ::std::mem::transmute::<&Self, &[T; 3]>(self) 
        }
    }
}

impl<T> AsMut<[T; 3]> for Rgb<T> where T: ColorChannel {
    fn as_mut(&mut self) -> &mut [T; 3] {
        unsafe {
            ::std::mem::transmute::<&mut Self, &mut [T; 3]>(self) 
        }
    }
}

impl<T, R> Add<R> for Rgb<T> where T: ColorChannel, R: Into<Rgb<T>> {
    type Output = Rgb<T>;
    fn add(self, other: R) -> Rgb<T> {
        let rhs = other.into();
        Self::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b)
    }
}

impl<T, R> Mul<R> for Rgb<T> where T: ColorChannel, R: Into<Rgb<T>> {
    type Output = Rgb<T>;
    fn mul(self, other: R) -> Rgb<T> {
        let rhs = other.into();
        Self::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b)
    }
}



#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Rgba<T = f32> where T: ColorChannel {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> where T: ColorChannel {
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Rgba {r, g, b, a}
    }
}

//impl<T> ColorBlend for Rgba<T>
//impl<T> ColorClamp for Rgba<T>
//impl<T> Into<[T; 4] for Rgba<T>
//impl<T> Into<[T; 3] for Rgba<T>
//impl<T> From[T; 4] for Rgba<T>
//impl<T> From[T; 3] for Rgba<T>
//impl<T, U> From<Rgba<T>> for Rgba<U>
//impl<T, U> From<Rgb<T>> for Rgba<U>
//impl<T, U> From<Luma<T>> for Rgba<U>
//impl<T, U> From<LumaA<T>> for Rgba<U>
//impl<T> From<T> for Rgba<T>
//impl<T> AsRef<[T; 4]> for Rgba<T>
//impl<T> AsMut<T; 4] for Rgba<T>
//impl<T, U> Add<U> for Rgba<T>
//impl<T, U> Mul<U> for Rgba<T>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Luma<T = f32> where T: ColorChannel {
    pub luma: T,
}

impl<T> Luma<T> where T: ColorChannel {
    pub const fn new(luma: T) -> Self {
        Luma {luma}
    }
}

//impl<T> ColorBlend for Luma<T>
//impl<T> ColorClamp for Luma<T>
//impl<T> Into<[T; 1]> for Luma<T>
//impl<T> Into<[T; 2]> for Luma<T>
//impl<T> From<[T; 1]> for Luma<T>
//impl<T> From<[T; 2]> for Luma<T>
//impl<T, U> From<Luma<T>> for Luma<U>
//impl<T, U> From<LumaA<T>> for Luma<U>
//impl<T> From<T> for Luma<T>
//impl<T> AsRef<[T; 1]> for Luma<T>
//impl<T> AsMUt<[T; 1]> for Luma<T>
//impl<T, U> Add<U> for Luma<T>
//impl<T, U> Mul<U> for Luma<T>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct LumaA<T = f32> where T: ColorChannel {
    pub luma: T,
    pub a: T,
}

impl<T> LumaA<T> where T: ColorChannel {
    pub const fn new(luma: T, a: T) -> Self {
        LumaA {luma, a}
    }
}

//impl<T> ColorBlend for LumaA<T>
//impl<T> ColorClamp for LumaA<T>
//impl<T> Into<[T; 1]> for LumaA<T>
//impl<T> Into<[T; 2]> for LumaA<T>
//impl<T> From<[T; 1]> for LumaA<T>
//impl<T> From<[T; 2]> for LumaA<T>
//impl<T, U> From<Luma<T>> for LumaA<U>
//impl<T, U> From<LumaA<T>> for LumaA<U>
//impl<T> From<T> for LumaA<T>
//impl<T> AsRef<[T; 2]> for LumaA<T>
//impl<T> AsMUt<[T; 2]> for LumaA<T>
//impl<T, U> Add<U> for LumaA<T>
//impl<T, U> Mul<U> for LumaA<T>