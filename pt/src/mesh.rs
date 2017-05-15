use traits::{Vertex, Material};
use polygon::Polygon;
use std::sync::Arc;

pub struct Mesh<V>
    where V: Vertex
{
    vertices: Vec<V>,
    indices: Vec<[u32; 3]>,
    materials: Vec<Arc<Material<V>>>,
}

impl<V> Mesh<V>
    where V: Vertex
{
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn from_data(vertices: Vec<V>,
                     indices: Vec<[u32; 3]>,
                     materials: Vec<Arc<Material<V>>>)
                     -> Result<Self, ()> {
        let vnum = vertices.len() as u32;
        let bounded = indices
            .iter()
            .fold(true,
                  |b, &[i0, i1, i2]| b && i0 < vnum && i1 < vnum && i2 < vnum);

        if bounded && indices.len() == materials.len() {
            Ok(Mesh {
                   vertices,
                   indices,
                   materials,
               })
        } else {
            Err(())
        }

    }

    pub fn add_vertex(&mut self, v: V) {
        self.vertices.push(v);
    }

    pub fn add_vertices<I>(&mut self, iter: I)
        where I: Iterator<Item = V>
    {
        for v in iter {
            self.vertices.push(v);
        }
    }

    pub fn add_face(&mut self, indices: [u32; 3], material: Arc<Material<V>>) -> Result<(), ()> {
        let vnum = self.vertices.len() as u32;
        if indices[0] < vnum && indices[1] < vnum && indices[2] < vnum {
            self.indices.push(indices);
            self.materials.push(material);

            Ok(())
        } else {
            Err(())
        }
    }

    // pub fn iter_mut<'s>(&'s mut self)
    //      ->  impl Iterator<Item = (&'s mut V, &'s mut V, &'s mut V, &'s mut Arc<Material<V>>)> + 's
    // {
    // }

    pub fn polygon_iter<'s>(&'s self) -> impl Iterator<Item = Polygon<'s, V>> + 's {
        let mat_iter = self.materials.iter();
        self.indices.iter().zip(mat_iter).map(move |(&[i0, i1, i2], mat)| {
            Polygon::new(self.vertices.get(i0 as usize).unwrap(),
                         self.vertices.get(i1 as usize).unwrap(),
                         self.vertices.get(i2 as usize).unwrap(),
                         mat.clone())
        })
    }

    pub fn polygons<'s>(&'s self) -> Vec<Polygon<'s, V>> {
        let mut pols = Vec::new();
        for (&[i0, i1, i2], mat) in self.indices.iter().zip(self.materials.iter()) {
            pols.push(Polygon::new(self.vertices.get(i0 as usize).unwrap(),
                                   self.vertices.get(i1 as usize).unwrap(),
                                   self.vertices.get(i2 as usize).unwrap(),
                                   mat.clone()));
        }
        pols
    }
}