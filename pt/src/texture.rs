use core::array::FixedSizeArray;
use std::marker::PhantomData;
use utils::{clamp};
use color::{Pixel};

pub struct Texture<P: Pixel<R>, R: FixedSizeArray<P::Channel> + Copy> {
    data: Vec<R>,
    width: usize,
    height: usize,
    _marker: PhantomData<P>,
}

impl<P: Pixel<R>, R: FixedSizeArray<P::Channel> + Copy> Texture<P, R>  {

    pub fn new(width: usize, height: usize) -> Self where P: Default {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(P::Color::default().into());
        }
        Texture::from_data(data, width, height)
    }
    
    pub fn from_data(data: Vec<R>, width: usize, height: usize) -> Self {
        Texture {
            data,
            width,
            height,
            _marker: PhantomData,
        }
    }

    pub fn pixel(&self, i: usize, j: usize) -> P::Color {
        self.data[self.width * j + i].into()
    }

    pub fn pixel_raw(&self, i: usize, j: usize) -> &R {
        &self.data[self.width * j + i]
    }

    pub fn set_pixel(&mut self, i: usize, j: usize, p: P::Color) {
        self.data[self.width * j + i] = p.into();
    }

    pub fn sample(&self, u: f32, v: f32) -> P::Color {
        let i = (clamp(u, 0.0, 1.0) * ((self.width - 1) as f32) + 0.5) as usize;
        let j = (clamp(v, 0.0, 1.0) * ((self.height - 1) as f32) + 0.5) as usize;
        self.pixel(i, j)
    }

    pub fn as_slice(&self) -> &[R] {
        self.data.as_slice()
    }

    pub fn pixels<'s>(&'s self) -> impl Iterator<Item = P::Color> + 's {
        self.data.iter().map(|&r| r.into())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
