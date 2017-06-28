use std::marker::PhantomData;
use utils::{clamp};
use color::{Pixel, RawColor};
use std::rc::Rc;
use std::sync::Arc;

pub trait Tex<C>: Sync + Send {
    fn new(width: usize, height: usize) -> Self where Self: Sized;
    fn pixel(&self, i: usize, j: usize) -> C;
    fn set_pixel(&mut self, i: usize, j: usize, p: C);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn sample(&self, u: f32, v: f32) -> C;
}

#[derive(Clone, Debug)]
pub struct Texture<P: Pixel<R>, R: RawColor<P::Channel>> {
    data: Vec<R>,
    width: usize,
    height: usize,
    _marker: PhantomData<P>,
}

impl<P: Pixel<R>, R: RawColor<P::Channel>> Texture<P, R>  {

    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(P::Color::default().into());
        }
        Texture::from_data(data, width, height)
    }
    
    pub fn from_data(data: Vec<R>, width: usize, height: usize) -> Self {
        assert!(data.len() == width * height);
        Texture {
            data,
            width,
            height,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn pixel(&self, i: usize, j: usize) -> P::Color {
        self.data[self.width * j + i].into()
    }

    #[inline]
    pub fn pixel_raw(&self, i: usize, j: usize) -> &R {
        &self.data[self.width * j + i]
    }

    #[inline]
    pub fn set_pixel(&mut self, i: usize, j: usize, p: P::Color) {
        self.data[self.width * j + i] = p.into();
    }

    #[inline]
    pub fn sample(&self, u: f32, v: f32) -> P::Color {
        let i = (clamp(u, 0.0, 1.0) * ((self.width - 1) as f32) + 0.5) as usize;
        let j = (clamp(v, 0.0, 1.0) * ((self.height - 1) as f32) + 0.5) as usize;
        self.pixel(i, j)
    }

    #[inline]
    pub fn as_slice(&self) -> &[R] {
        self.data.as_slice()
    }

    #[inline]
    pub fn pixels<'s>(&'s self) -> impl Iterator<Item = P::Color> + 's {
        self.data.iter().map(|&r| r.into())
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<P, R, C> Tex<C> for Texture<P, R> 
    where P: Pixel<R>, 
          R: RawColor<P::Channel>,
          C: Into<P::Color>,
          P::Color: Into<C>
{
    #[inline]
    fn new(width: usize, height: usize) -> Self where Self: Sized {
        Texture::new(width, height)
    }
    #[inline]
    fn pixel(&self, i: usize, j: usize) -> C {
        self.pixel(i, j).into()
    }
    #[inline]
    fn set_pixel(&mut self, i: usize, j: usize, p: C) {
        self.set_pixel(i, j, p.into());
    }
    #[inline]
    fn width(&self) -> usize {
        self.width
    }
    #[inline]
    fn height(&self) -> usize {
        self.height
    }
    #[inline]
    fn sample(&self, u: f32, v: f32) -> C {
        self.sample(u, v).into()
    }
}

impl<'a, P, R, C> AsRef<Tex<C> + 'a> for Texture<P, R>
    where P: Pixel<R> + 'a, 
          R: RawColor<P::Channel> + 'a,
          C: Into<P::Color> + 'a,
          P::Color: Into<C> + 'a
{
    #[inline]
    fn as_ref(&self) -> &(Tex<C> + 'a) {
        self
    }
}

impl<'a, P, R, C> AsMut<Tex<C> + 'a> for Texture<P, R>
    where P: Pixel<R> + 'a, 
          R: RawColor<P::Channel> + 'a,
          C: Into<P::Color> + 'a,
          P::Color: Into<C> + 'a
{
    #[inline]
    fn as_mut(&mut self) -> &mut (Tex<C> + 'a) {
        self
    }
}

impl<'s, 'a: 's, C> AsRef<Tex<C> + 'a> for &'s (Tex<C> + 'a) {
    #[inline]
    fn as_ref(&self) -> &(Tex<C> + 'a) {
        *self
    }
}

impl<'s, 'a: 's, C> AsMut<Tex<C> + 'a> for &'s mut (Tex<C> + 'a) {
    #[inline]
    fn as_mut(&mut self) -> &mut (Tex<C> + 'a) {
        *self
    }
}

impl<'a, P, R, C> AsRef<Tex<C> + 'a> for Box<Texture<P, R>>
    where P: Pixel<R> + 'a, 
          R: RawColor<P::Channel> + 'a,
          C: Into<P::Color> + 'a,
          P::Color: Into<C> + 'a
{
    #[inline]
    fn as_ref(&self) -> &(Tex<C> + 'a) {
        &**self
    }
}

impl<'a, P, R, C> AsMut<Tex<C> + 'a> for Box<Texture<P, R>>
    where P: Pixel<R> + 'a, 
          R: RawColor<P::Channel> + 'a,
          C: Into<P::Color> + 'a,
          P::Color: Into<C> + 'a
{
    #[inline]
    fn as_mut(&mut self) -> &mut (Tex<C> + 'a) {
        &mut **self
    }
}