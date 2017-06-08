pub mod spheres;

use pt::math::{Point3f, Real};
use pt::polygon::{TexturedVertex, DiffuseTex};
use pt::mesh::{Mesh};
use pt::texture::Texture;
use pt::color::Rgb;

pub mod meshes {
    use pt::math::{Point3f, Real};
    use pt::polygon::{BaseVertex, Vertex, Material, DiffuseMat, TexturedVertex, DiffuseTex};
    use pt::mesh::{Mesh};
    use pt::color;
    use std::sync::Arc;
    use std::collections::BTreeMap;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum CubeSide {
        Top = 0,
        Bottom,
        Right,
        Left,
        Front,
        Back,        
    }

    pub struct Cube<'a, V: Vertex> {
        pub center: Point3f,
        pub size: (Real, Real, Real),
        pub materials: BTreeMap<CubeSide, Arc<Material<V> + 'a>>,
    }

    impl<'a, V: Vertex> Cube<'a, V> {
        pub fn new(center: Point3f, size: (Real, Real, Real)) -> Cube<'a, V> {
            let mat = Arc::new(DiffuseMat::new(color::WHITE, None)) as Arc<Material<V>>;
            Cube {
                center,
                size,
                materials: {
                    let mut m = BTreeMap::new();
                    m.insert(CubeSide::Top, mat.clone());
                    m.insert(CubeSide::Bottom, mat.clone());
                    m.insert(CubeSide::Right, mat.clone());
                    m.insert(CubeSide::Left, mat.clone());                    
                    m.insert(CubeSide::Front, mat.clone());
                    m.insert(CubeSide::Back, mat.clone());
                    m
                }
            }
        }

        pub fn with_materials<F>(center: Point3f, size: (Real, Real, Real), mut get_mat: F) -> Cube<'a, V>
            where F: FnMut(CubeSide) -> Arc<Material<V> + 'a>
        {
            Cube {
                center,
                size,
                materials: {
                    let mut m = BTreeMap::new();
                    m.insert(CubeSide::Top, get_mat(CubeSide::Top));
                    m.insert(CubeSide::Bottom, get_mat(CubeSide::Bottom));
                    m.insert(CubeSide::Right, get_mat(CubeSide::Right));
                    m.insert(CubeSide::Left, get_mat(CubeSide::Left));                    
                    m.insert(CubeSide::Front, get_mat(CubeSide::Front));
                    m.insert(CubeSide::Back, get_mat(CubeSide::Back));
                    m
                }
            }
        }
    }

    impl<'a, T: Vertex> Cube<'a, T> {

        // vertex order(normal is directed towards the observer):
        // 1-----2
        // |   / |
        // | /   |
        // 0-----3
        pub fn build<F>(&self, invert_normals: bool, mut pos_to_vertex: F) -> Mesh<'a, T> 
            where F: FnMut(CubeSide, &[Point3f; 4]) -> [T; 4]
        {
            use self::CubeSide::*;
            let mut mesh = Mesh::new();

            let center = self.center;
            let (dx, dy, dz) = {
                let (x, y, z) = self.size;
                (x * 0.5, y * 0.5, z * 0.5)
            };

            let top = [
                Point3f::new(center.x - dx, center.y + dy, center.z + dz),
                Point3f::new(center.x - dx, center.y + dy, center.z - dz),
                Point3f::new(center.x + dx, center.y + dy, center.z - dz),
                Point3f::new(center.x + dx, center.y + dy, center.z + dz),];

            let bottom = [
                Point3f::new(center.x - dx, center.y - dy, center.z - dz),
                Point3f::new(center.x - dx, center.y - dy, center.z + dz),
                Point3f::new(center.x + dx, center.y - dy, center.z + dz),
                Point3f::new(center.x + dx, center.y - dy, center.z - dz)];

            let right = [
                Point3f::new(center.x + dx, center.y - dy, center.z + dz),
                Point3f::new(center.x + dx, center.y + dy, center.z + dz),
                Point3f::new(center.x + dx, center.y + dy, center.z - dz),
                Point3f::new(center.x + dx, center.y - dy, center.z - dz)];

            let left = [
                Point3f::new(center.x - dx, center.y - dy, center.z - dz),
                Point3f::new(center.x - dx, center.y + dy, center.z - dz),
                Point3f::new(center.x - dx, center.y + dy, center.z + dz),
                Point3f::new(center.x - dx, center.y - dy, center.z + dz)];

            let front = [
                Point3f::new(center.x - dx, center.y - dy, center.z + dz),
                Point3f::new(center.x - dx, center.y + dy, center.z + dz),
                Point3f::new(center.x + dx, center.y + dy, center.z + dz),
                Point3f::new(center.x + dx, center.y - dy, center.z + dz)];
            
            let back = [
                Point3f::new(center.x - dx, center.y + dy, center.z - dz),
                Point3f::new(center.x - dx, center.y - dy, center.z - dz),
                Point3f::new(center.x + dx, center.y - dy, center.z - dz),
                Point3f::new(center.x + dx, center.y + dy, center.z - dz),];

            {
                let mut add_side = |side: CubeSide, i: u32, verts: &[Point3f; 4]| {                
                    let vertices = pos_to_vertex(side, verts);
                    for v in vertices.into_iter() {
                        mesh.add_vertex(*v);
                    }
                    let ix = i * 4;
                    if !invert_normals {
                        mesh.add_face([ix + 0, ix + 1, ix + 2], self.materials.get(&side).unwrap().clone()).unwrap();
                        mesh.add_face([ix + 0, ix + 2, ix + 3], self.materials.get(&side).unwrap().clone()).unwrap();
                    } else {
                        mesh.add_face([ix + 0, ix + 2, ix + 1], self.materials.get(&side).unwrap().clone()).unwrap();
                        mesh.add_face([ix + 0, ix + 3, ix + 2], self.materials.get(&side).unwrap().clone()).unwrap();
                    }
                };

                add_side(Top, 0, &top);
                add_side(Bottom, 1, &bottom);
                add_side(Right, 2, &right);
                add_side(Left, 3, &left);
                add_side(Front, 4, &front);
                add_side(Back, 5, &back);

            }

            mesh
            
        }
    }

    impl Cube<'static, BaseVertex> {
        pub fn with_bv(center: Point3f, size: (Real, Real, Real)) -> Cube<'static, BaseVertex> {
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

pub fn cube_with_diffuse_tex<'a>(pos: Point3f, 
                                 size: (Real, Real, Real), 
                                 albedo: &'a Texture<Rgb, [f32; 3]>,
                                 emittance: Option<&'a Texture<Rgb, [f32; 3]>>) 
                                 -> Mesh<'a, TexturedVertex>
{
    use self::meshes::Cube;
    use std::sync::Arc;
    use pt::math::{Point2};
    let mat = Arc::new(DiffuseTex::new(albedo, emittance));
    let cube = Cube::with_materials(pos, size, |_| mat.clone());
    cube.build(false, |_, v| {
        [
            TexturedVertex::new(v[0], Point2::new(0.0, 0.0)),
            TexturedVertex::new(v[1], Point2::new(0.0, 1.0)),
            TexturedVertex::new(v[2], Point2::new(1.0, 1.0)),
            TexturedVertex::new(v[3], Point2::new(1.0, 0.0)),
        ]
    })
}