use math::{Ray3f};
use traits::{Surface, SceneHolder, Material};
use ::{SurfacePoint};
use std::ops::Deref;
use std::convert::AsRef;
use std;
use rand::random;


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
        let mut t_min: f32 = std::f32::MAX;
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
      
        sp
    }

    fn random_light_source<'s> (&'s self) -> Option<&'s Surface> {
        if self.light_sources.len() > 0 {
            //let ix = random::<usize>() % self.light_sources.len();
            //Some(self.light_sources[ix].as_ref())
            Some(self.light_sources[0].as_ref())
        } else {
            None
        }
    }
}