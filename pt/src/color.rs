use core::array::FixedSizeArray;

use texture;
use num::{One, Zero, FromPrimitive, ToPrimitive, Bounded};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::u8;
use utils::{clamp};

pub use self::consts::*;

pub type Color = Rgb;
pub type Image = texture::Texture<f32, [f32; 4], Rgb>;

mod consts {
    use super::Color;
    pub const ALICEBLUE: Color =        Color::new(240 as f32, 248 as f32, 255 as f32);
    pub const ANTIQUEWHITE: Color =     Color::new(250 as f32, 235 as f32, 215 as f32);
    pub const AQUA: Color =             Color::new(0 as f32, 255 as f32, 255 as f32);
    pub const AQUAMARINE: Color =       Color::new(127 as f32, 255 as f32, 212 as f32);
    pub const AZURE: Color =            Color::new(240 as f32, 255 as f32, 255 as f32);
    pub const BEIGE: Color =            Color::new(245 as f32, 245 as f32, 220 as f32);
    pub const BISQUE: Color =           Color::new(255 as f32, 228 as f32, 196 as f32);
    pub const BLACK: Color =            Color::new(0 as f32, 0 as f32, 0 as f32);
    pub const BLANCHEDALMOND: Color =   Color::new(255 as f32, 235 as f32, 205 as f32);
    pub const BLUE: Color =             Color::new(0 as f32, 0 as f32, 255 as f32);
    pub const BLUEVIOLET: Color =       Color::new(138 as f32, 43 as f32, 226 as f32);
    pub const BROWN: Color =            Color::new(165 as f32, 42 as f32, 42 as f32);
    pub const BURLYWOOD: Color =        Color::new(222 as f32, 184 as f32, 135 as f32);
    pub const CADETBLUE: Color =        Color::new(95 as f32, 158 as f32, 160 as f32);
    pub const CHARTREUSE: Color =       Color::new(127 as f32, 255 as f32, 0 as f32);
    pub const CHOCOLATE: Color =        Color::new(210 as f32, 105 as f32, 30 as f32);
    pub const CORAL: Color =            Color::new(255 as f32, 127 as f32, 80 as f32);
    pub const CORNFLOWERBLUE: Color =   Color::new(100 as f32, 149 as f32, 237 as f32);
    pub const CORNSILK: Color =         Color::new(255 as f32, 248 as f32, 220 as f32);
    pub const CRIMSON: Color =          Color::new(220 as f32, 20 as f32, 60 as f32);
    pub const CYAN: Color =             Color::new(0 as f32, 255 as f32, 255 as f32);
    pub const DARKBLUE: Color =         Color::new(0 as f32, 0 as f32, 139 as f32);
    pub const DARKCYAN: Color =         Color::new(0 as f32, 139 as f32, 139 as f32);
    pub const DARKGOLDENROD: Color =    Color::new(184 as f32, 134 as f32, 11 as f32);
    pub const DARKGRAY: Color =         Color::new(169 as f32, 169 as f32, 169 as f32);
    pub const DARKGREEN: Color =        Color::new(0 as f32, 100 as f32, 0 as f32);
    pub const DARKGREY: Color =         Color::new(169 as f32, 169 as f32, 169 as f32);
    pub const DARKKHAKI: Color =        Color::new(189 as f32, 183 as f32, 107 as f32);
    pub const DARKMAGENTA: Color =      Color::new(139 as f32, 0 as f32, 139 as f32);
    pub const DARKOLIVEGREEN: Color =   olor::new(85 as f32, 107 as f32, 47 as f32);
    pub const DARKORANGE: Color =       Color::new(255 as f32, 140 as f32, 0 as f32);
    pub const DARKORCHID: Color =       Color::new(153 as f32, 50 as f32, 204 as f32);
    pub const DARKRED: Color =          Color::new(139 as f32, 0 as f32, 0 as f32);
    pub const DARKSALMON: Color =       Color::new(233 as f32, 150 as f32, 122 as f32);
    pub const DARKSEAGREEN: Color =     Color::new(143 as f32, 188 as f32, 143 as f32);
    pub const DARKSLATEBLUE: Color =    Color::new(72 as f32, 61 as f32, 139 as f32);
    pub const DARKSLATEGRAY: Color =    Color::new(47 as f32, 79 as f32, 79 as f32);
    pub const DARKSLATEGREY: Color =    Color::new(47 as f32, 79 as f32, 79 as f32);
    pub const DARKTURQUOISE: Color =    Color::new(0 as f32, 206 as f32, 209 as f32);
    pub const DARKVIOLET: Color =       Color::new(148 as f32, 0 as f32, 211 as f32);
    pub const DEEPPINK: Color =         Color::new(255 as f32, 20 as f32, 147 as f32);
    pub const DEEPSKYBLUE: Color =      Color::new(0 as f32, 191 as f32, 255 as f32);
    pub const DIMGRAY: Color =          Color::new(105 as f32, 105 as f32, 105 as f32);
    pub const DIMGREY: Color =          Color::new(105 as f32, 105 as f32, 105 as f32);
    pub const DODGERBLUE: Color =       Color::new(30 as f32, 144 as f32, 255 as f32);
    pub const FIREBRICK: Color =        Color::new(178 as f32, 34 as f32, 34 as f32);
    pub const FLORALWHITE: Color =      Color::new(255 as f32, 250 as f32, 240 as f32);
    pub const FORESTGREEN: Color =      Color::new(34 as f32, 139 as f32, 34 as f32);
    pub const FUCHSIA: Color =          Color::new(255 as f32, 0 as f32, 255 as f32);
    pub const GAINSBORO: Color =        Color::new(220 as f32, 220 as f32, 220 as f32);
    pub const GHOSTWHITE: Color =       Color::new(248 as f32, 248 as f32, 255 as f32);
    pub const GOLD: Color =             Color::new(255 as f32, 215 as f32, 0 as f32);
    pub const GOLDENROD: Color =        Color::new(218 as f32, 165 as f32, 32 as f32);
    pub const GRAY: Color =             Color::new(128 as f32, 128 as f32, 128 as f32);
    pub const GREY: Color =             Color::new(128 as f32, 128 as f32, 128 as f32);
    pub const GREEN: Color =            Color::new(0 as f32, 128 as f32, 0 as f32);
    pub const GREENYELLOW: Color =      Color::new(173 as f32, 255 as f32, 47 as f32);
    pub const HONEYDEW: Color =         Color::new(240 as f32, 255 as f32, 240 as f32);
    pub const HOTPINK: Color =          Color::new(255 as f32, 105 as f32, 180 as f32);
    pub const INDIANRED: Color =        Color::new(205 as f32, 92 as f32, 92 as f32);
    pub const INDIGO: Color =           Color::new(75 as f32, 0 as f32, 130 as f32);
    pub const IVORY: Color =            Color::new(255 as f32, 255 as f32, 240 as f32);
    pub const KHAKI: Color =            Color::new(240 as f32, 230 as f32, 140 as f32);
    pub const LAVENDER: Color =         Color::new(230 as f32, 230 as f32, 250 as f32);
    pub const LAVENDERBLUSH: Color =    Color::new(255 as f32, 240 as f32, 245 as f32);
    pub const LAWNGREEN: Color =        Color::new(124 as f32, 252 as f32, 0 as f32);
    pub const LEMONCHIFFON: Color =     Color::new(255 as f32, 250 as f32, 205 as f32);
    pub const LIGHTBLUE: Color =        Color::new(173 as f32, 216 as f32, 230 as f32);
    pub const LIGHTCORAL: Color =       Color::new(240 as f32, 128 as f32, 128 as f32);
    pub const LIGHTCYAN: Color =        Color::new(224 as f32, 255 as f32, 255 as f32);
    pub const LIGHTGOLDENRODYELLOW: Color = Color::new(250 as f32, 250 as f32, 210 as f32);
    pub const LIGHTGRAY: Color =        Color::new(211 as f32, 211 as f32, 211 as f32);
    pub const LIGHTGREEN: Color =       Color::new(144 as f32, 238 as f32, 144 as f32);
    pub const LIGHTGREY: Color =        Color::new(211 as f32, 211 as f32, 211 as f32);
    pub const LIGHTPINK: Color =        Color::new(255 as f32, 182 as f32, 193 as f32);
    pub const LIGHTSALMON: Color =      Color::new(255 as f32, 160 as f32, 122 as f32);
    pub const LIGHTSEAGREEN: Color =    Color::new(32 as f32, 178 as f32, 170 as f32);
    pub const LIGHTSKYBLUE: Color =     Color::new(135 as f32, 206 as f32, 250 as f32);
    pub const LIGHTSLATEGRAY: Color =   Color::new(119 as f32, 136 as f32, 153 as f32);
    pub const LIGHTSLATEGREY: Color =   Color::new(119 as f32, 136 as f32, 153 as f32);
    pub const LIGHTSTEELBLUE: Color =   Color::new(176 as f32, 196 as f32, 222 as f32);
    pub const LIGHTYELLOW: Color =      Color::new(255 as f32, 255 as f32, 224 as f32);
    pub const LIME: Color =             Color::new(0 as f32, 255 as f32, 0 as f32);
    pub const LIMEGREEN: Color =        Color::new(50 as f32, 205 as f32, 50 as f32);
    pub const LINEN: Color =            Color::new(250 as f32, 240 as f32, 230 as f32);
    pub const MAGENTA: Color =          Color::new(255 as f32, 0 as f32, 255 as f32);
    pub const MAROON: Color =           Color::new(128 as f32, 0 as f32, 0 as f32);
    pub const MEDIUMAQUAMARINE: Color = Color::new(102 as f32, 205 as f32, 170 as f32);
    pub const MEDIUMBLUE: Color =       Color::new(0 as f32, 0 as f32, 205 as f32);
    pub const MEDIUMORCHID: Color =     Color::new(186 as f32, 85 as f32, 211 as f32);
    pub const MEDIUMPURPLE: Color =     Color::new(147 as f32, 112 as f32, 219 as f32);
    pub const MEDIUMSEAGREEN: Color =   Color::new(60 as f32, 179 as f32, 113 as f32);
    pub const MEDIUMSLATEBLUE: Color =  Color::new(123 as f32, 104 as f32, 238 as f32);
    pub const MEDIUMSPRINGGREEN: Color = Color::new(0 as f32, 250 as f32, 154 as f32);
    pub const MEDIUMTURQUOISE: Color =  Color::new(72 as f32, 209 as f32, 204 as f32);
    pub const MEDIUMVIOLETRED: Color =  Color::new(199 as f32, 21 as f32, 133 as f32);
    pub const MIDNIGHTBLUE: Color =     Color::new(25 as f32, 25 as f32, 112 as f32);
    pub const MINTCREAM: Color =        Color::new(245 as f32, 255 as f32, 250 as f32);
    pub const MISTYROSE: Color =        Color::new(255 as f32, 228 as f32, 225 as f32);
    pub const MOCCASIN: Color =         Color::new(255 as f32, 228 as f32, 181 as f32);
    pub const NAVAJOWHITE: Color =      Color::new(255 as f32, 222 as f32, 173 as f32);
    pub const NAVY: Color =             Color::new(0 as f32, 0 as f32, 128 as f32);
    pub const OLDLACE: Color =          Color::new(253 as f32, 245 as f32, 230 as f32);
    pub const OLIVE: Color =            Color::new(128 as f32, 128 as f32, 0 as f32);
    pub const OLIVEDRAB: Color =        Color::new(107 as f32, 142 as f32, 35 as f32);
    pub const ORANGE: Color =           Color::new(255 as f32, 165 as f32, 0 as f32);
    pub const ORANGERED: Color =        Color::new(255 as f32, 69 as f32, 0 as f32);
    pub const ORCHID: Color =           Color::new(218 as f32, 112 as f32, 214 as f32);
    pub const PALEGOLDENROD: Color =    Color::new(238 as f32, 232 as f32, 170 as f32);
    pub const PALEGREEN: Color =        Color::new(152 as f32, 251 as f32, 152 as f32);
    pub const PALETURQUOISE: Color =    Color::new(175 as f32, 238 as f32, 238 as f32);
    pub const PALEVIOLETRED: Color =    Color::new(219 as f32, 112 as f32, 147 as f32);
    pub const PAPAYAWHIP: Color =       Color::new(255 as f32, 239 as f32, 213 as f32);
    pub const PEACHPUFF: Color =        Color::new(255 as f32, 218 as f32, 185 as f32);
    pub const PERU: Color =             Color::new(205 as f32, 133 as f32, 63 as f32);
    pub const PINK: Color =             Color::new(255 as f32, 192 as f32, 203 as f32);
    pub const PLUM: Color =             Color::new(221 as f32, 160 as f32, 221 as f32);
    pub const POWDERBLUE: Color =       Color::new(176 as f32, 224 as f32, 230 as f32);
    pub const PURPLE: Color =           Color::new(128 as f32, 0 as f32, 128 as f32);
    pub const REBECCAPURPLE: Color =    Color::new(102 as f32, 51 as f32, 153 as f32);
    pub const RED: Color =              Color::new(255 as f32, 0 as f32, 0 as f32);
    pub const ROSYBROWN: Color =        Color::new(188 as f32, 143 as f32, 143 as f32);
    pub const ROYALBLUE: Color =        Color::new(65 as f32, 105 as f32, 225 as f32);
    pub const SADDLEBROWN: Color =      Color::new(139 as f32, 69 as f32, 19 as f32);
    pub const SALMON: Color =           Color::new(250 as f32, 128 as f32, 114 as f32);
    pub const SANDYBROWN: Color =       Color::new(244 as f32, 164 as f32, 96 as f32);
    pub const SEAGREEN: Color =         Color::new(46 as f32, 139 as f32, 87 as f32);
    pub const SEASHELL: Color =         Color::new(255 as f32, 245 as f32, 238 as f32);
    pub const SIENNA: Color =           Color::new(160 as f32, 82 as f32, 45 as f32);
    pub const SILVER: Color =           Color::new(192 as f32, 192 as f32, 192 as f32);
    pub const SKYBLUE: Color =          Color::new(135 as f32, 206 as f32, 235 as f32);
    pub const SLATEBLUE: Color =        Color::new(106 as f32, 90 as f32, 205 as f32);
    pub const SLATEGRAY: Color =        Color::new(112 as f32, 128 as f32, 144 as f32);
    pub const SLATEGREY: Color =        Color::new(112 as f32, 128 as f32, 144 as f32);
    pub const SNOW: Color =             Color::new(255 as f32, 250 as f32, 250 as f32);
    pub const SPRINGGREEN: Color =      Color::new(0 as f32, 255 as f32, 127 as f32);
    pub const STEELBLUE: Color =        Color::new(70 as f32, 130 as f32, 180 as f32);
    pub const TAN: Color =              Color::new(210 as f32, 180 as f32, 140 as f32);
    pub const TEAL: Color =             Color::new(0 as f32, 128 as f32, 128 as f32);
    pub const THISTLE: Color =          Color::new(216 as f32, 191 as f32, 216 as f32);
    pub const TOMATO: Color =           Color::new(255 as f32, 99 as f32, 71 as f32);
    pub const TURQUOISE: Color =        Color::new(64 as f32, 224 as f32, 208 as f32);
    pub const VIOLET: Color =           Color::new(238 as f32, 130 as f32, 238 as f32);
    pub const WHEAT: Color =            Color::new(245 as f32, 222 as f32, 179 as f32);
    pub const WHITE: Color =            Color::new(255 as f32, 255 as f32, 255 as f32);
    pub const WHITESMOKE: Color =       Color::new(245 as f32, 245 as f32, 245 as f32);
    pub const YELLOW: Color =           Color::new(255 as f32, 255 as f32, 0 as f32);
    pub const YELLOWGREEN: Color =      Color::new(154 as f32, 205 as f32, 50 as f32);

}

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