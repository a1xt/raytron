pub struct Polygon<'a, T: BaseTypes> {
    v0: &'a Vertex<T>,
    v1: &'a Vertex<T>,
    v2: &'a Vertex<T>,
}

pub struct Vertex<T: BaseTypes> {
    position: Point3<T::Real>,
    bsdf: Box<Bsdf<T::LightIntensity>>,
}

impl<T: BaseTypes> Polygon<T> {
    pub fn bsdf(&self, uv: (T::Real, T::Real, T::Real)) -> Box<Bsdf<T::LightIntensity>>;
}

//impl Surface for Polygon {}



pub struct Mesh<T: BaseTypes> {
    indices: Vec<u32>,
    vertices: Vec<Vertex<T>>,
}

impl<T: BaseTypes> Mesh {
    pub fn iter(&'s self) -> impl Iterator<Item = Polygon<'s, T>> {
        MeshIter {
            ind: &self.indices,
            vert: &self.vertices,
            cur_poly: 0, 
        }
    }
}

pub struct MeshIter<'a, T: BaseTypes> {
    ind: &'a Vec<u32>,
    vert: &'a Vec<Vertex<T>>,
    cur_poly: usize,
}

impl<'a, T: Configtypes> Iterator for MeshIter<'a, T> {
    type Item = Polygon<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct MeshList<T: BaseTypes> {
    meshes: Vec<Box<Mesh<T>>>,
}
//impl SceneHolder for MeshList { }