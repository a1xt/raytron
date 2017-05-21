pub mod spheres;


pub mod meshes {
    use pt::math::{Point3f, Real};
    use pt::polygon::{BaseVertex, Vertex, Material, DiffuseMat};
    use pt::mesh::{Mesh};
    use pt::color;
    use std::sync::Arc;
    use std::collections::BTreeMap;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum CubeSide {
        Top,
        Bottom,
        Left,
        Right,
        Front,
        Back,        
    }

    pub struct Cube<V: Vertex> {
        pub center: Point3f,
        pub size: (Real, Real, Real),
        pub materials: BTreeMap<CubeSide, Arc<Material<V>>>,
    }

    impl Cube<BaseVertex> {
        pub fn with_bv(center: Point3f, size: (Real, Real, Real)) -> Cube<BaseVertex> {
            Cube {
                center,
                size,
                materials: {
                    let mut m = BTreeMap::new();
                    m.insert(CubeSide::Top, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m.insert(CubeSide::Bottom, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m.insert(CubeSide::Left, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m.insert(CubeSide::Right, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m.insert(CubeSide::Front, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m.insert(CubeSide::Back, Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<BaseVertex>>);
                    m
                }
            }
        }

        pub fn build_bv(&self, invert_normals: bool) -> Mesh<BaseVertex> {
            use self::CubeSide::*;
            let mut mesh = Mesh::new();

            let center = self.center;
            let (dx, dy, dz) = {
                let (x, y, z) = self.size;
                (x * 0.5, y * 0.5, z * 0.5)
            };
            let vertices = vec! [
                BaseVertex::new(Point3f::new(center.x - dx, center.y - dy, center.z + dz)),
                BaseVertex::new(Point3f::new(center.x - dx, center.y - dy, center.z - dz)),
                BaseVertex::new(Point3f::new(center.x + dx, center.y - dy, center.z - dz)),
                BaseVertex::new(Point3f::new(center.x + dx, center.y - dy, center.z + dz)),
                BaseVertex::new(Point3f::new(center.x - dx, center.y + dy, center.z + dz)),
                BaseVertex::new(Point3f::new(center.x - dx, center.y + dy, center.z - dz)),
                BaseVertex::new(Point3f::new(center.x + dx, center.y + dy, center.z - dz)),
                BaseVertex::new(Point3f::new(center.x + dx, center.y + dy, center.z + dz)),

            ];
            
            for v in vertices.iter() {
                mesh.add_vertex(*v);
            }

            {
                let mut face = |side: CubeSide, i0: u32, i1: u32, i2: u32| {
                    if let Some(mat) = self.materials.get(&side) {
                        if !invert_normals {
                            mesh.add_face([i0, i1, i2], mat.clone()).unwrap();
                        } else {
                            mesh.add_face([i0, i2, i1], mat.clone()).unwrap();
                        }
                    }
                };

                face(Top, 4, 5, 6);     face(Top, 4, 6, 7);
                face(Bottom, 0, 2, 1);  face(Bottom, 0, 3, 2);
                face(Left, 0, 1, 4);    face(Left, 1, 5, 4);
                face(Right, 2, 3, 7);   face(Right, 2, 7, 6);
                face(Front, 0, 4, 7);   face(Front, 0, 7, 3);
                face(Back, 1, 6, 5);    face(Back, 1, 2, 6);
            }

            mesh
        }
    }
}