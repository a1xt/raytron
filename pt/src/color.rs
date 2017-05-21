use core::array::FixedSizeArray;

use texture;
use num::{One, Zero, FromPrimitive, ToPrimitive, Bounded};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::u8;
use utils::{clamp};

pub use self::consts::*;

pub type Color = Rgb;
pub type Image = texture::Texture<Rgb, [f32; 4]>;

mod consts {
    use super::Color;

    macro_rules! color {
        ($c:ident, $($x:expr),+) => {
            pub const $c: Color = Color::new(
                $(
                    ($x as f32) / 255.0,
                )*
            );
        }
    }
    color!(ALICEBLUE, 240, 248, 255);
    color!(ANTIQUEWHITE, 250, 235, 215);
    color!(AQUA, 0, 255, 255);
    color!(AQUAMARINE, 127, 255, 212);
    color!(AZURE, 240, 255, 255);
    color!(BEIGE, 245, 245, 220);
    color!(BISQUE, 255, 228, 196);
    color!(BLACK, 0, 0, 0);
    color!(BLANCHEDALMOND, 255, 235, 205);
    color!(BLUE, 0, 0, 255);
    color!(BLUEVIOLET, 138, 43, 226);
    color!(BROWN, 165, 42, 42);
    color!(BURLYWOOD, 222, 184, 135);
    color!(CADETBLUE, 95, 158, 160);
    color!(CHARTREUSE, 127, 255, 0);
    color!(CHOCOLATE, 210, 105, 30);
    color!(CORAL, 255, 127, 80);
    color!(CORNFLOWERBLUE, 100, 149, 237);
    color!(CORNSILK, 255, 248, 220);
    color!(CRIMSON, 220, 20, 60);
    color!(CYAN, 0, 255, 255);
    color!(DARKBLUE, 0, 0, 139);
    color!(DARKCYAN, 0, 139, 139);
    color!(DARKGOLDENROD, 184, 134, 11);
    color!(DARKGRAY, 169, 169, 169);
    color!(DARKGREEN, 0, 100, 0);
    color!(DARKGREY, 169, 169, 169);
    color!(DARKKHAKI, 189, 183, 107);
    color!(DARKMAGENTA, 139, 0, 139);
    color!(DARKOLIVEGREEN, 85, 107, 47);
    color!(DARKORANGE, 255, 140, 0);
    color!(DARKORCHID, 153, 50, 204);
    color!(DARKRED, 139, 0, 0);
    color!(DARKSALMON, 233, 150, 122);
    color!(DARKSEAGREEN, 143, 188, 143);
    color!(DARKSLATEBLUE, 72, 61, 139);
    color!(DARKSLATEGRAY, 47, 79, 79);
    color!(DARKSLATEGREY, 47, 79, 79);
    color!(DARKTURQUOISE, 0, 206, 209);
    color!(DARKVIOLET, 148, 0, 211);
    color!(DEEPPINK, 255, 20, 147);
    color!(DEEPSKYBLUE, 0, 191, 255);
    color!(DIMGRAY, 105, 105, 105);
    color!(DIMGREY, 105, 105, 105);
    color!(DODGERBLUE, 30, 144, 255);
    color!(FIREBRICK, 178, 34, 34);
    color!(FLORALWHITE, 255, 250, 240);
    color!(FORESTGREEN, 34, 139, 34);
    color!(FUCHSIA, 255, 0, 255);
    color!(GAINSBORO, 220, 220, 220);
    color!(GHOSTWHITE, 248, 248, 255);
    color!(GOLD, 255, 215, 0);
    color!(GOLDENROD, 218, 165, 32);
    color!(GRAY, 128, 128, 128);
    color!(GREY, 128, 128, 128);
    color!(GREEN, 0, 128, 0);
    color!(GREENYELLOW, 173, 255, 47);
    color!(HONEYDEW, 240, 255, 240);
    color!(HOTPINK, 255, 105, 180);
    color!(INDIANRED, 205, 92, 92);
    color!(INDIGO, 75, 0, 130);
    color!(IVORY, 255, 255, 240);
    color!(KHAKI, 240, 230, 140);
    color!(LAVENDER, 230, 230, 250);
    color!(LAVENDERBLUSH, 255, 240, 245);
    color!(LAWNGREEN, 124, 252, 0);
    color!(LEMONCHIFFON, 255, 250, 205);
    color!(LIGHTBLUE, 173, 216, 230);
    color!(LIGHTCORAL, 240, 128, 128);
    color!(LIGHTCYAN, 224, 255, 255);
    color!(LIGHTGOLDENRODYELLOW, 250, 250, 210);
    color!(LIGHTGRAY, 211, 211, 211);
    color!(LIGHTGREEN, 144, 238, 144);
    color!(LIGHTGREY, 211, 211, 211);
    color!(LIGHTPINK, 255, 182, 193);
    color!(LIGHTSALMON, 255, 160, 122);
    color!(LIGHTSEAGREEN, 32, 178, 170);
    color!(LIGHTSKYBLUE, 135, 206, 250);
    color!(LIGHTSLATEGRAY, 119, 136, 153);
    color!(LIGHTSLATEGREY, 119, 136, 153);
    color!(LIGHTSTEELBLUE, 176, 196, 222);
    color!(LIGHTYELLOW, 255, 255, 224);
    color!(LIME, 0, 255, 0);
    color!(LIMEGREEN, 50, 205, 50);
    color!(LINEN, 250, 240, 230);
    color!(MAGENTA, 255, 0, 255);
    color!(MAROON, 128, 0, 0);
    color!(MEDIUMAQUAMARINE, 102, 205, 170);
    color!(MEDIUMBLUE, 0, 0, 205);
    color!(MEDIUMORCHID, 186, 85, 211);
    color!(MEDIUMPURPLE, 147, 112, 219);
    color!(MEDIUMSEAGREEN, 60, 179, 113);
    color!(MEDIUMSLATEBLUE, 123, 104, 238);
    color!(MEDIUMSPRINGGREEN, 0, 250, 154);
    color!(MEDIUMTURQUOISE, 72, 209, 204);
    color!(MEDIUMVIOLETRED, 199, 21, 133);
    color!(MIDNIGHTBLUE, 25, 25, 112);
    color!(MINTCREAM, 245, 255, 250);
    color!(MISTYROSE, 255, 228, 225);
    color!(MOCCASIN, 255, 228, 181);
    color!(NAVAJOWHITE, 255, 222, 173);
    color!(NAVY, 0, 0, 128);
    color!(OLDLACE, 253, 245, 230);
    color!(OLIVE, 128, 128, 0);
    color!(OLIVEDRAB, 107, 142, 35);
    color!(ORANGE, 255, 165, 0);
    color!(ORANGERED, 255, 69, 0);
    color!(ORCHID, 218, 112, 214);
    color!(PALEGOLDENROD, 238, 232, 170);
    color!(PALEGREEN, 152, 251, 152);
    color!(PALETURQUOISE, 175, 238, 238);
    color!(PALEVIOLETRED, 219, 112, 147);
    color!(PAPAYAWHIP, 255, 239, 213);
    color!(PEACHPUFF, 255, 218, 185);
    color!(PERU, 205, 133, 63);
    color!(PINK, 255, 192, 203);
    color!(PLUM, 221, 160, 221);
    color!(POWDERBLUE, 176, 224, 230);
    color!(PURPLE, 128, 0, 128);
    color!(REBECCAPURPLE, 102, 51, 153);
    color!(RED, 255, 0, 0);
    color!(ROSYBROWN, 188, 143, 143);
    color!(ROYALBLUE, 65, 105, 225);
    color!(SADDLEBROWN, 139, 69, 19);
    color!(SALMON, 250, 128, 114);
    color!(SANDYBROWN, 244, 164, 96);
    color!(SEAGREEN, 46, 139, 87);
    color!(SEASHELL, 255, 245, 238);
    color!(SIENNA, 160, 82, 45);
    color!(SILVER, 192, 192, 192);
    color!(SKYBLUE, 135, 206, 235);
    color!(SLATEBLUE, 106, 90, 205);
    color!(SLATEGRAY, 112, 128, 144);
    color!(SLATEGREY, 112, 128, 144);
    color!(SNOW, 255, 250, 250);
    color!(SPRINGGREEN, 0, 255, 127);
    color!(STEELBLUE, 70, 130, 180);
    color!(TAN, 210, 180, 140);
    color!(TEAL, 0, 128, 128);
    color!(THISTLE, 216, 191, 216);
    color!(TOMATO, 255, 99, 71);
    color!(TURQUOISE, 64, 224, 208);
    color!(VIOLET, 238, 130, 238);
    color!(WHEAT, 245, 222, 179);
    color!(WHITE, 255, 255, 255);
    color!(WHITESMOKE, 245, 245, 245);
    color!(YELLOW, 255, 255, 0);
    color!(YELLOWGREEN, 154, 205, 50);

}

pub trait ColorChannel: Copy + PartialEq + PartialOrd + One + Zero + ChannelBounds + ChannelBlend + 
                        FromPrimitive + ToPrimitive + Bounded + Default {}
impl ColorChannel for u8 {}
impl ColorChannel for f32 {}

pub trait ChannelBounds {
    const MIN_CHVAL: Self;
    const MAX_CHVAL: Self;
}

impl ChannelBounds for u8 {
    const MIN_CHVAL: u8 = 0;
    const MAX_CHVAL: u8 = u8::MAX;
}
impl ChannelBounds for f32 {
    const MIN_CHVAL: f32 = 0.0;
    const MAX_CHVAL: f32 = 1.0;
}

pub trait ChannelBlend {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self;
}

impl ChannelBlend for f32 {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self {
        c0 * w0 + c1 * w1
    }
}

impl ChannelBlend for u8 {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self {
        let t = w0 * (c0 as f32) + w1 * (c1 as f32);
        clamp(t, 0.0, u8::MAX_CHVAL as f32) as u8
    }
}

pub trait ColorClamp {
    fn clamp(self) -> Self;
}

pub trait ColorBlend<T: ColorChannel> {
    fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self;
}

pub trait Pixel<R> where R: FixedSizeArray<Self::Channel> + Copy {
    type Channel: ColorChannel;
    type Color: Copy + ColorBlend<Self::Channel> + From<R> + Into<R> + Default;
}

pub trait ChannelCast<T: ColorChannel>: ColorChannel {
    fn cast_from(other: T) -> Self;
    fn cast_into(self) -> T;
}

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


macro_rules! impl_color {
    ($c:ident, $($x:ident),+) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
        pub struct $c<T = f32> where T: ColorChannel {
            $(
                pub $x: T,
            )*
        }

        impl<T: ColorChannel> $c<T> {
            pub const fn new($( $x: T, )* ) -> Self {
                Self {
                    $( $x, )*
                }
            }
        }
    }
}

macro_rules! impl_pixel {
    ($c:ident, [$t:ident; $n:expr]) => {
        impl<$t: ColorChannel> Pixel<[$t; $n]> for $c<$t> {
            type Channel = T;
            type Color = Self;
        }
    }
}

macro_rules! impl_from_self {
    ($c:ident, $t:ident, $u:ident, $( $x:ident ),+) => {
        impl From<$c<$t>> for $c<$u> {
            fn from(other: $c<$t>) -> Self {
                Self::new(
                    $(
                        other.$x.cast_into(),
                    )*
                )
            }
        }
    }
}

macro_rules! impl_colorblend {
    ($c:ident, $($x:ident),+) => {
        impl<T> ColorBlend<T> for $c<T> where T: ColorChannel {
            fn blend(c0: Self, w0: f32, c1: Self, w1: f32) -> Self {
                Self::new(
                    $(
                        ChannelBlend::blend(c0.$x, w0, c1.$x, w1),
                    )*
                )
            }            
        }
    }
}

macro_rules! impl_colorclamp {
    ($c:ident, $( $x:ident ),+) => {
        impl<T> ColorClamp for $c<T> where T: ColorChannel {
            fn clamp(self) -> Self {
                Self::new(
                    $(
                        clamp(self.$x, T::MIN_CHVAL, T::MAX_CHVAL),
                    )*
                )
            }
        }
    }
}

macro_rules! impl_from_arr {
    ($c:ident, [$t:ident; $n:expr], $v:ident, $($x:expr),+) => {
        impl<$t> From<[$t; $n]> for $c<$t> where $t: ColorChannel {
            fn from($v: [$t; $n]) -> Self {
                Self::new(
                    $(
                        $x,
                    )*
                )
            }
        }
    }
}


macro_rules! impl_into_arr_a {
    ($c:ident, [$t:ident; $n:expr], $($x:ident),+) => {
        impl<$t> Into<[$t; $n]> for $c<$t> where $t: ColorChannel {
            fn into(self) -> [$t; $n] {
                [$(
                    self.$x,
                )*
                $t::MAX_CHVAL
                ]
            }
        }
    };
}
macro_rules! impl_into_arr {
    ($c:ident, [$t:ident; $n:expr], $($x:ident),+ ) => {
        impl<$t> Into<[$t; $n]> for $c<$t> where $t: ColorChannel {
            fn into(self) -> [$t; $n] {
                [$(
                    self.$x,
                )*
                ]
            }
        }
    };
    
}


macro_rules! impl_from_other {
    ($c:ident, $r:ident, $($x:ident),+) => {
        impl<T, U> From<$r<T>> for $c<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
            fn from(other: $r<T>) -> Self {
                Self::new(
                    $(
                        other.$x.cast_into(),
                    )*
                )
            }
        }
    }
}

macro_rules! impl_from_other_a {
    ($c:ident, $r:ident, $($x:ident),+) => {
        impl<T, U> From<$r<T>> for $c<U> where T: ColorChannel + ChannelCast<U>, U: ColorChannel {
            fn from(other: $r<T>) -> Self {
                Self::new(
                    $(
                        other.$x.cast_into(),
                    )*
                    U::MAX_CHVAL
                )
            }
        }
    }
}

macro_rules! impl_from_scalar {
    ($c:ident, $($x:ident),+) => {
        impl<T> From<T> for $c<T> where T: ColorChannel {
            fn from(val: T) -> Self {
                Self {
                    $($x: val, )*
                }
            }
        }
    }
}

macro_rules! impl_from_scalar_a{
    ($c:ident, $($x:ident),+) => {
        impl<T> From<T> for $c<T> where T: ColorChannel {
            fn from(val: T) -> Self {
                Self {
                    $($x: val, )*
                    a: T::MAX_CHVAL
                }
            }
        }
    }
}

macro_rules! impl_asref {
    ($c:ident, [$t:ident; $n:expr]) => {
        impl<$t> AsRef<[$t; $n]> for $c<$t> where $t: ColorChannel {
            fn as_ref(&self) -> &[$t; $n] {
                unsafe {
                    ::std::mem::transmute::<&Self, &[$t; $n]>(self) 
                }
            }
        }
    }
}

macro_rules! impl_asmut {
    ($c:ident, [$t:ident; $n:expr]) => {
        impl<$t> AsMut<[$t; $n]> for $c<$t> where $t: ColorChannel {
            fn as_mut(&mut self) -> &mut [$t; $n] {
                unsafe {
                    ::std::mem::transmute::<&mut Self, &mut [$t; $n]>(self) 
                }
            }
        }
    }
}


macro_rules! impl_add {
    ($c:ident, $($x:ident),+) => {
        impl<T, R> Add<R> for $c<T> where T: ColorChannel, R: Into<$c<T>> {
            type Output = $c<T>;
            fn add(self, other: R) -> Self::Output {
                let rhs = other.into();
                Self::new(
                    $(
                        self.$x + rhs.$x,
                    )*
                )
            }
        }
        impl<T, R> AddAssign<R> for $c<T> where T: ColorChannel, R: Into<$c<T>> {
            fn add_assign(&mut self, other: R) {
                let rhs = other.into();
                *self = Self::new(
                    $(
                        self.$x + rhs.$x,
                    )*
                )
            }
        }
    }
}

macro_rules! impl_mul {
    ($c:ident, $($x:ident),+) => {
        impl<T, R> Mul<R> for $c<T> where T: ColorChannel, R: Into<$c<T>> {
            type Output = $c<T>;
            fn mul(self, other: R) -> Self::Output {
                let rhs = other.into();
                Self::new(
                    $(
                        self.$x * rhs.$x,
                    )*
                )
            }
        }
        impl<T, R> MulAssign<R> for $c<T> where T: ColorChannel, R: Into<$c<T>> {
            fn mul_assign(&mut self, other: R) {
                let rhs = other.into();
                *self = Self::new(
                    $(
                        self.$x * rhs.$x,
                    )*
                )
            }
        }
    }
}


impl_color!(Rgb, r, g, b);

impl_pixel!(Rgb, [T; 4]);
impl_pixel!(Rgb, [T; 3]);
impl_colorblend!(Rgb, r, g, b);
impl_colorclamp!(Rgb, r, g, b);
impl_from_arr!(Rgb, [T; 3], v, v[0], v[1], v[2]);
impl_from_arr!(Rgb, [T; 4], v, v[0], v[1], v[2]);
impl_into_arr!(Rgb, [T; 3], r, g, b);
impl_into_arr_a!(Rgb, [T; 4], r, g, b);
impl_from_self!(Rgb, u8, f32, r, g, b);
impl_from_self!(Rgb, f32, u8, r, g, b);
impl_from_other!(Rgb, Rgba, r, g, b);
impl_from_other!(Rgb, Luma, luma, luma, luma);
impl_from_other!(Rgb, LumaA, luma, luma, luma);
impl_from_scalar!(Rgb, r, g, b);
impl_asref!(Rgb, [T; 3]);
impl_asmut!(Rgb, [T; 3]);
impl_add!(Rgb, r, g, b);
impl_mul!(Rgb, r, g, b);


impl_color!(Rgba, r, g, b, a);

impl_pixel!(Rgba, [T; 4]);
impl_pixel!(Rgba, [T; 3]);
impl_colorblend!(Rgba, r, g, b, a);
impl_colorclamp!(Rgba, r, g, b, a);
impl_from_arr!(Rgba, [T; 3], v, v[0], v[1], v[2], T::MAX_CHVAL);
impl_from_arr!(Rgba, [T; 4], v, v[0], v[1], v[2], v[3]);
impl_into_arr!(Rgba, [T; 3], r, g, b);
impl_into_arr!(Rgba, [T; 4], r, g, b, a);
impl_from_self!(Rgba, u8, f32, r, g, b, a);
impl_from_self!(Rgba, f32, u8, r, g, b, a);
impl_from_other_a!(Rgba, Rgb, r, g, b);
impl_from_other_a!(Rgba, Luma, luma, luma, luma);
impl_from_other!(Rgba, LumaA, luma, luma, luma, a);
impl_from_scalar_a!(Rgba, r, g, b);
impl_asref!(Rgba, [T; 4]);
impl_asmut!(Rgba, [T; 4]);
impl_add!(Rgba, r, g, b, a);
impl_mul!(Rgba, r, g, b, a);


impl_color!(Luma, luma);

impl_pixel!(Luma, [T; 1]);
impl_pixel!(Luma, [T; 2]);
impl_colorblend!(Luma, luma);
impl_colorclamp!(Luma, luma);
impl_from_arr!(Luma, [T; 1], v, v[0]);
impl_from_arr!(Luma, [T; 2], v, v[0]);
impl_into_arr!(Luma, [T; 1], luma);
impl_into_arr_a!(Luma, [T; 2], luma);
impl_from_self!(Luma, u8, f32, luma);
impl_from_self!(Luma, f32, u8, luma);
impl_from_other!(Luma, LumaA, luma);
impl_from_scalar!(Luma, luma);
impl_asref!(Luma, [T; 1]);
impl_asmut!(Luma, [T; 1]);
impl_add!(Luma, luma);
impl_mul!(Luma, luma);



impl_color!(LumaA, luma, a);

impl_pixel!(LumaA, [T; 1]);
impl_pixel!(LumaA, [T; 2]);
impl_colorblend!(LumaA, luma, a);
impl_colorclamp!(LumaA, luma, a);
impl_from_arr!(LumaA, [T; 1], v, v[0], T::MAX_CHVAL);
impl_from_arr!(LumaA, [T; 2], v, v[0], v[1]);
impl_into_arr!(LumaA, [T; 1], luma);
impl_into_arr!(LumaA, [T; 2], luma, a);
impl_from_self!(LumaA, u8, f32, luma, a);
impl_from_self!(LumaA, f32, u8, luma, a);
impl_from_other_a!(LumaA, Luma, luma);
impl_from_scalar_a!(LumaA, luma);
impl_asref!(LumaA, [T; 2]);
impl_asmut!(LumaA, [T; 2]);
impl_add!(LumaA, luma, a);
impl_mul!(LumaA, luma, a);