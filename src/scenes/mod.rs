pub mod spheres;

use pt::math::{Point3f, Vector3f, Real, Norm, Cross};
use pt::polygon::{Vertex, Material};
use pt::mesh::{Mesh};
use pt::utils::consts;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CubeSide {
    Top = 0,
    Bottom,
    Right,
    Left,
    Front,
    Back,
}

// 1-----2
// |   / |
// | /   |
// 0-----3
#[derive(Clone)]
pub struct Quad<V: Clone> {
    pub v0: V,
    pub v1: V,
    pub v2: V,
    pub v3: V,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RotOrder {
    CW,
    CCW,
}

impl Default for RotOrder {
    fn default() -> Self {
        RotOrder::CW
    }
}

// pub struct Plane<V: Vertex + Clone> {
//     center: Point3f,
//     normal: Vector3f,
//     up: Vector3f,
//     size: (Real, Real),
//     subdiv: (usize, usize),
//     mat: Arc<Material<V>>,
//     front_face: RotOrder,
//     map_quads: RefCell<Box<FnMut(Quad<Point3f>) -> Quad<V> + 'static>>,
// }

pub struct Plane;

impl Plane {
    pub fn build<'m, 'c, F, V>(
        center: Point3f,
        look_at: Point3f,
        up: Vector3f,
        size: (Real, Real),
        mat: Arc<Material<V> + 'm>,
        subdiv: Option<(usize, usize)>,
        front_face: Option<RotOrder>,
        mut map_quads: F)
        -> Mesh<'m, V>
        where F: FnMut(Quad<Point3f>) -> Quad<V> + 'c,
              V: Vertex + 'm
    {
        let front_face = front_face.unwrap_or(RotOrder::CW);
        let (w, h) = size;
        let (subdiv_x, subdiv_y) = subdiv.unwrap_or((1, 1));
        assert!(subdiv_x > 0 && subdiv_y > 0);

        let normal = (look_at - center).normalize();
        let right = up.cross(&normal).normalize();
        let up = normal.cross(&right).normalize();      
        let dx = 1.0 / (subdiv_x as Real);
        let dy = 1.0 / (subdiv_y as Real);
        let v0 = center - right * (w * 0.5) - up * (h * 0.5);

        let outer_quad = map_quads(Quad {
            v0: v0,
            v1: v0 + up * h,
            v2: v0 + up * h + right * w,
            v3: v0 + right * w,
        });

        let mut mesh = Mesh::with_capacity((subdiv_x + 1) * (subdiv_y + 1), subdiv_x * subdiv_y * 2);

        for j in 0..(subdiv_y + 1) {
            for i in 0..(subdiv_x + 1) {
                let x = (i as Real) * dx;
                let y = (j as Real) * dy;
                let v = if x + y < 1.0 + consts::REAL_EPSILON {
                    <V as Vertex>::interpolate(
                        &outer_quad.v0,
                        &outer_quad.v3,
                        &outer_quad.v1,                        
                        (1.0 - (x + y), x, y),
                    )
                } else {
                    <V as Vertex>::interpolate(
                        &outer_quad.v2,
                        &outer_quad.v1,
                        &outer_quad.v3,                                                
                        (x + y - 1.0, 1.0 - x, 1.0 - y),
                    )
                };
                mesh.add_vertex(v);
            }
        }

        for j in 0..subdiv_y {
            for i in 0..subdiv_x {
                let ix0 = (i + j * (subdiv_x + 1)) as u32;
                let ix1 = (i + (j + 1) * (subdiv_x + 1)) as u32;
                let ix2 = (i + 1 + (j + 1) * (subdiv_x + 1)) as u32;
                let ix3 = (i + 1 + j * (subdiv_x + 1)) as u32;

                let (f1, f2) = if let RotOrder::CW = front_face {
                    ([ix0, ix1, ix2], [ix0, ix2, ix3])
                } else {
                    ([ix0, ix2, ix1], [ix0, ix3, ix2])
                };

                mesh.add_face(f1, mat.clone()).unwrap();
                mesh.add_face(f2, mat.clone()).unwrap();
            }
        }

        mesh
    }

}


pub struct Cube;

impl Cube {
    pub fn build<'c, 'm, V, F, G, M>(
        center: Point3f, 
        size: Vector3f,
        mut map_quads: F,
        mut map_materials: M,
        mut map_subdivs: G,
        invert_normals: bool)
        -> Mesh<'m, V>
        where F: FnMut(CubeSide, Quad<Point3f>) -> Quad<V> + 'c,
              G: FnMut(CubeSide) -> (usize, usize) + 'c,
              M: FnMut(CubeSide) -> Arc<Material<V> + 'm> + 'c,
              V: Vertex + 'm

    {
        assert!(size.x > 0.0 && size.y > 0.0 && size.z > 0.0);
        // let mut map_subdivs = if let Some(ms) = map_subdivs {
        //     box ms as Box<FnMut<_, Output = _>>
        // } else {
        //     box |_| {(1, 1)} 
        // };

        let sides = [
            CubeSide::Front,
            CubeSide::Back,
            CubeSide::Right,
            CubeSide::Left,
            CubeSide::Top,
            CubeSide::Bottom,
        ];
    
        let c = center;
        let dx = (size * 0.5).x;
        let dy = (size * 0.5).y;
        let dz = (size * 0.5).z;
        let front_face = if !invert_normals { RotOrder::CW } else {RotOrder::CCW };
        let mut planes = Vec::with_capacity(sides.len());
        for &side in &sides {
            let (origin, up, size) = match side {
                CubeSide::Front => (Point3f::new(c.x, c.y, c.z + dz), Vector3f::new(0.0, 1.0, 0.0), (size.x, size.y)),
                CubeSide::Back => (Point3f::new(c.x, c.y, c.z - dz), Vector3f::new(0.0, 1.0, 0.0), (size.x, size.y)),
                CubeSide::Right => (Point3f::new(c.x + dx, c.y, c.z), Vector3f::new(0.0, 1.0, 0.0), (size.z, size.y)),
                CubeSide::Left => (Point3f::new(c.x - dx, c.y, c.z), Vector3f::new(0.0, 1.0, 0.0), (size.z, size.y)),
                CubeSide::Top => (Point3f::new(c.x, c.y + dy, c.z), Vector3f::new(0.0, 0.0, -1.0), (size.x, size.z)),
                CubeSide::Bottom => (Point3f::new(c.x, c.y - dy, c.z), Vector3f::new(0.0, 0.0, 1.0), (size.x, size.z)),
            };

            let normal = origin - center;
            let mut plane = Plane::build(
                origin,
                origin + normal,
                up,
                size,
                map_materials(side),
                Some(map_subdivs(side)),
                Some(front_face),
                |quad| { map_quads(side, quad) });
            
            planes.push(plane);
        }

        let mut mesh = Mesh::new();
        for mut p in planes {
            mesh.merge(&mut p);
        }
        
        mesh
    }    
}