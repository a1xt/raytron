use math::{Ray3f, Real};
use traits::{Surface, SceneHandler};
use {SurfacePoint};
use std;
use std::sync::Arc;
use std::marker::PhantomData;
use utils::consts;
use super::{LightSourcesHandler, UniformSampler, LuminairesSampler};

pub struct ShapeListBuilder<'a, T, S = UniformSampler<'a>> 
    where T: AsRef<Surface + 'a> + Sync + 'a,
          S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a  
{
    shapes: Vec<T>,
    light_sources: Vec<&'a Surface>,
    _marker: PhantomData<S>,
}

impl<'a, T, S> ShapeListBuilder<'a, T, S> 
    where  T: AsRef<Surface + 'a> + Sync + 'a,
           S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a
{
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            light_sources: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn add_shape(&mut self, surface: T) {
        if surface.as_ref().is_emitter() {
            self.shapes.push(surface);
            let s_ref = self.shapes.last().unwrap().as_ref();
            unsafe{
                self.light_sources.push(::std::mem::transmute(s_ref));
            }           
        } else {
            self.shapes.push(surface);
        }
    }

    pub fn shape_list(&self) -> ShapeList<'a, T, S> where T: Clone {
        ShapeList {
            shapes: self.shapes.clone(),
            light_sources: self.light_sources.clone(),
            sampler: Arc::new(S::from(self.light_sources.as_slice())),
        }
    }

    pub fn to_shape_list(self) -> ShapeList<'a, T, S> {
        let sampler = unsafe { Arc::new(S::from(self.light_sources.as_slice())) };
        ShapeList {
            shapes: self.shapes,
            light_sources: self.light_sources,
            sampler,
        }
    }
}

pub struct ShapeList<'a, T, S = UniformSampler<'a>> 
    where T: AsRef<Surface + 'a> + Sync + 'a,
          S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a,
{
    shapes: Vec<T>,
    light_sources: Vec<&'a (Surface + 'a)>,
    sampler: Arc<S>,
}

impl<'a, T, S> SceneHandler for ShapeList<'a, T, S>
    where T: AsRef<Surface + 'a> + Sync + 'a,
          S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a,
{
    fn intersection(&self, ray: &Ray3f) -> Option<SurfacePoint> {
        let mut t_min: Real = std::f32::MAX as Real;
        let mut sp = None;

        for shape in self.shapes.iter() {
            if let Some((t, surf_point)) = shape.as_ref().intersection(ray) {
                if t < t_min {
                    t_min = t;
                    sp = Some(surf_point);
                }
            }
        }

        if let Some(ref mut x) = sp {
            x.position += x.normal * consts::POSITION_EPSILON;
        }
      
        sp
    }
    
    fn light_sources_iter<'s>(&'s self) -> Box<Iterator<Item = &'s Surface> + 's> {
        box self.light_sources.iter().cloned()
    }

    fn light_sources<'s>(&'s self) -> LightSourcesHandler<'s> {
        LightSourcesHandler {
            scene: self,
            sampler: super::lt_arc_trait_hack(self.sampler.clone()),
            //sampler: self.sampler.clone(),
        }
    }
}