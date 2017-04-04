use math::{Ray3f, Real};
use traits::{Surface, SceneHolder};
use ::{SurfacePoint};
use std::convert::AsRef;
use std;
use utils::consts;

pub struct ShapeList<'a> {
    shapes: Vec<Box<Surface + 'a>>,
    light_sources: Vec<Box<Surface + 'a>>,
}

impl<'a> ShapeList<'a> {
    pub fn new() -> ShapeList<'a> {
        ShapeList {
            shapes: Vec::new(),
            light_sources: Vec::new(),
        }
    }

    pub fn add_shape<S: Surface + 'a> (&mut self, shape: S, light_source: bool) {
        if light_source {
            self.light_sources.push(Box::new(shape));
        } else {
            self.shapes.push(Box::new(shape));
        }
    }
}

impl<'a> SceneHolder for ShapeList<'a> {
    fn intersection_with_scene(&self, ray: &Ray3f) -> Option<SurfacePoint> {
        let mut t_min: Real = std::f32::MAX as Real;
        let mut sp = None;

        for shape in self.shapes.iter() {
            if let Some((t, surf_point)) = shape.intersection(ray) {
                if t < t_min {
                    t_min = t;
                    sp = Some(surf_point);
                }
            }
        }

        for shape in self.light_sources.iter() {
            if let Some((t, surf_point)) = shape.intersection(ray) {
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
        Box::new(self.light_sources.iter().map(|s| s.as_ref()))
    }
}