use std::marker::PhantomData;
use utils::{clamp};
use color::{Pixel, RawColor};
use std::rc::Rc;
use std::sync::Arc;

pub trait TexView<P>: Sync + Send {
    fn new(width: usize, height: usize) -> Self where Self: Sized;
    fn pixel(&self, i: usize, j: usize) -> P;
    fn set_pixel(&mut self, i: usize, j: usize, p: P);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn sample(&self, u: f32, v: f32) -> P;
}

#[derive(Clone, Debug)]
//pub struct Texture<P: Pixel<R>, R: RawColor<P::Channel>> {
pub struct Texture<P: Pixel> {
    data: Vec<P::Raw>,
    width: usize,
    height: usize,
    //_marker: PhantomData<P>,
}

impl<P: Pixel> Texture<P> {

    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(P::Color::default().into());
        }
        Texture::from_data(data, width, height)
    }
    
    pub fn from_data(data: Vec<P::Raw>, width: usize, height: usize) -> Self {
        assert!(data.len() == width * height);
        Texture {
            data,
            width,
            height,
            //_marker: PhantomData,
        }
    }

    #[inline]
    pub fn pixel(&self, i: usize, j: usize) -> P::Color {
        self.data[self.width * j + i].into()
    }

    #[inline]
    pub fn pixel_raw(&self, i: usize, j: usize) -> &P::Raw {
        &self.data[self.width * j + i]
    }

    #[inline]
    pub fn set_pixel_raw(&mut self, i: usize, j: usize, p: P::Raw) {
        self.data[self.width * j + i] = p;
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
    pub fn as_slice(&self) -> &[P::Raw] {
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

impl<P, R> TexView<P> for Texture<R> 
    where R: Pixel,
          P: From<R::Raw> + Into<R::Raw> + From<R::Color>,
{
    #[inline]
    fn new(width: usize, height: usize) -> Self where Self: Sized {
        Texture::new(width, height)
    }

    #[inline]
    fn pixel(&self, i: usize, j: usize) -> P {
        P::from(*self.pixel_raw(i, j))
    }
    #[inline]
    fn set_pixel(&mut self, i: usize, j: usize, p: P) {
        self.set_pixel_raw(i, j, p.into());
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
    fn sample(&self, u: f32, v: f32) -> P {
        P::from(self.sample(u, v))
    }
}

impl<'a, R, P> AsRef<TexView<P> + 'a> for Texture<R>
    where R: Pixel + 'a,
          P: From<R::Raw> + Into<R::Raw> + From<R::Color>,
{
    #[inline]
    fn as_ref(&self) -> &(TexView<P> + 'a) {
        self
    }
}

impl<'a, P, R> AsMut<TexView<P> + 'a> for Texture<R>
    where R: Pixel + 'a,
          P: From<R::Raw> + Into<R::Raw> + From<R::Color>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut (TexView<P> + 'a) {
        self
    }
}

impl<'s, 'a: 's, P> AsRef<TexView<P> + 'a> for &'s (TexView<P> + 'a) {
    #[inline]
    fn as_ref(&self) -> &(TexView<P> + 'a) {
        *self
    }
}

impl<'s, 'a: 's, C> AsMut<TexView<C> + 'a> for &'s mut (TexView<C> + 'a) {
    #[inline]
    fn as_mut(&mut self) -> &mut (TexView<C> + 'a) {
        *self
    }
}

impl<'a, P, R> AsRef<TexView<P> + 'a> for Box<Texture<R>>
    where R: Pixel + 'a,
          P: From<R::Raw> + Into<R::Raw> + From<R::Color>,
{
    #[inline]
    fn as_ref(&self) -> &(TexView<P> + 'a) {
        &**self
    }
}

impl<'a, P, R> AsMut<TexView<P> + 'a> for Box<Texture<R>>
    where R: Pixel + 'a,
          P: From<R::Raw> + Into<R::Raw> + From<R::Color>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut (TexView<P> + 'a) {
        &mut **self
    }
}