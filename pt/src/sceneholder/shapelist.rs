use math::{Ray3f, Real};
use traits::{Surface, SceneHolder};
use {SurfacePoint};
use std;
use utils::consts;
use super::{LightSourcesHandler};

pub struct ShapeList<'a, T> where T: AsRef<Surface + 'a> + Sync + 'a {
    shapes: Vec<T>,
    light_sources: Vec<&'a (Surface + 'a)>,
}

impl<'a, T> ShapeList<'a, T> where  T: AsRef<Surface + 'a> + Sync + 'a {
    pub fn new() -> Self {
        ShapeList {
            shapes: Vec::new(),
            light_sources: Vec::new(),
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
}

impl<'a, T> SceneHolder for ShapeList<'a, T> where T: AsRef<Surface + 'a> + Sync + 'a {
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
        }
    }
}