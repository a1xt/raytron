pub mod shapelist;
pub mod kdtree;

pub use self::shapelist::ShapeList;

use traits::Surface;
use math::{Ray3f};
use {SurfacePoint};

pub trait SceneHolder: Sync {
    fn intersection(&self, ray: &Ray3f) -> Option<SurfacePoint>;

    // delete
    //fn random_light_source<'s>(&'s self) -> Option<&'s Surface>;
    
    //fn ligth_sources<'s>(&'s self) -> &'s[&'s Surface];
    //fn light_sources<'s, T>(&'s self) -> T where T: Iterator<Item=&'s Surface>;
    //fn light_sources<'s>(&'s self) -> Box<SurfacesIter<Item = &'s Surface>>;
    //fn light_sources<'s>(&'s self) -> &'s [AsRef<Surface + 's>];
    
    fn light_sources_iter<'s>(&'s self) -> Box<Iterator<Item = &'s Surface> + 's>;

    fn light_sources<'s>(&'s self) -> LightSourcesHandler<'s>;
}

#[derive(Clone)]
pub struct LightSourcesHandler<'a> {
    scene: &'a SceneHolder,
}

impl<'a> IntoIterator for LightSourcesHandler<'a> {
    type Item = &'a Surface;
    type IntoIter = Box<Iterator<Item = &'a Surface> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.scene.light_sources_iter()
    }
}