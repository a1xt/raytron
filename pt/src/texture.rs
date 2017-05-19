use core::array::FixedSizeArray;
use std::marker::PhantomData;
use utils::{clamp};
use color::{ColorChannel, Pixel};


pub struct Texture<T, R, P> where T: ColorChannel, R: FixedSizeArray<T> + Copy, P: Pixel<T, R> {
    data: Vec<R>,
    width: usize,
    height: usize,
    _marker_p: PhantomData<P>,
    _marker_t: PhantomData<T>,
}

impl<T, R, P> Texture<T, R, P> where T: ColorChannel, R: FixedSizeArray<T> + Copy, P: Pixel<T, R> {
    pub fn new(width: usize, height: usize) -> Self where P: Default {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(P::default().into());
        }
        Texture::from_data(data, width, height)
    }
    pub fn from_data(data: Vec<R>, width: usize, height: usize) -> Self {
        Texture {
            data,
            width,
            height,
            _marker_p: PhantomData,
            _marker_t: PhantomData,
        }
    }

    pub fn pixel(&self, i: usize, j: usize) -> P {
        self.data[self.width * j + i].into()
    }

    pub fn pixel_raw(&self, i: usize, j: usize) -> &R {
        &self.data[self.width * j + i]
    }

    pub fn set_pixel(&mut self, i: usize, j: usize, p: P) {
        self.data[self.width * j + i] = p.into();
    }

    pub fn sample(&self, u: f32, v: f32) -> P {
        let i = (clamp(u, 0.0, 1.0) * (self.width as f32) + 0.5) as usize;
        let j = (clamp(v, 0.0, 1.0) * (self.height as f32) + 0.5) as usize;
        self.pixel(i, j)
    }

    pub fn as_slice(&self) -> &[R] {
        self.data.as_slice()
    }

    pub fn pixels<'s>(&'s self) -> impl Iterator<Item = P> + 's {
        self.data.iter().map(|&r| r.into())
    }
}
