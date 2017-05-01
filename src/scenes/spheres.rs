use pt::sceneholder::{ShapeList};
use pt::bsdf::{Diffuse, Phong};
use pt::{Sphere, Color};
use pt::traits::{Surface};
use pt::math::{Vector3f, Point3f, Real};
use camera_controller::{FPSCameraController};
use std::collections::BTreeMap;
use std::sync::Arc;

    //use pt::polygon::*;
    use pt::color;

pub struct Room {
    pub spheres: BTreeMap<&'static str, Sphere>,  
}

impl Room {
    pub fn new() -> Self {
        
        let mut btmap = BTreeMap::new();

        //Left
        btmap.insert(
            "left",
            Sphere::new(
                Point3f::new(-1.0e5 - 50.0, 0.0, 0.0),
                1.0e5,
                Arc::new(Diffuse::new(Color{data: [0.75, 0.25, 0.25f32, 1.0]}, None)),
            ),
        );
        //Right
        btmap.insert(
            "right",
            Sphere::new(
                Point3f::new(1.0e5 + 50.0, 0.0, 0.0),
                1.0e5, ////
                Arc::new(Diffuse::new(Color{data: [0.25, 0.25, 0.75f32, 1.0]}, None)),
                //Arc::new(Phong::new(Color{data: [0.25, 0.25, 0.75f32, 1.0]}, 0.1, 0.9, 200.0)),
            ),            
        );
        //Back
        btmap.insert(
            "back",
            Sphere::new(
                Point3f::new(0.0, 0.0, 1.0e5 + 50.0),
                1.0e5,
                Arc::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
                //Arc::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.1, 0.900, 100000.0))
            )
        );
        //Front
        btmap.insert(
            "front",
            Sphere::new(
                Point3f::new(0.0, 0.0, -1.0e5 - 50.0),
                1.0e5,///////
                Arc::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
                //Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.1, 0.9, 100000.0))
            ),
        );
        //Bottom
        btmap.insert(
            "bottom",
            Sphere::new(
                Point3f::new(0.0, -1.0e5 - 50.0, 0.0),
                1.0e5,
                Arc::new(Diffuse::new(Color{data: [0.9, 0.9, 0.9f32, 1.0]}, None)),
                //Arc::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.2, 0.8, 3.0)),
            ),
        );
        //Top
        btmap.insert(
            "top",
            Sphere::new(
                Point3f::new(0., 1.0e5 + 50.0, 0.),
                1.0e5,
                Arc::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
                //Box::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, Some(Color{data: [1.5, 1.5, 1.5, 1.0]}))),
            ),
        );
        //Sphere 1
        //let k = 10.0;
        btmap.insert(
            "sphere1",
            Sphere::new(
                Point3f::new(-20., -35.0, -20.),
                7.0,
                Arc::new(Diffuse::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, None)),
                //Arc::new(Diffuse::new(Color{data: [0.9, 0.9, 0.9f32, 1.0]}, Some(Color{data: [0.2 * k, 0.5 * k, 0.2 * k, 1.0]}))),
                //Box::new(Diffuse::new(Color{data: [1.0, 1.0, 1.0f32, 1.0]}, Some(Color{data: [15.0, 15.0, 15.0f32, 1.0]}))),
            ),
        );
        //Sphere 2
        btmap.insert(
            "sphere2",
            Sphere::new(
                Point3f::new(15.0, -35.0, 5.0),
                15.,
                Arc::new(Diffuse::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, None)),
                //Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.3, 0.7, 100000)),
            ),
        );
        //Light source
        btmap.insert(
            "light1",
            Sphere::new(
                Point3f::new(0.0, 39.0, 0.0),
                10.0,
                Arc::new(Diffuse::new(Color{data: [1.0, 1.0, 1.0f32, 1.0]}, Some(Color{data: [15.0, 15.0, 15.0f32, 1.0]}))),
                //Box::new(Diffuse::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, None)),
            ),
        );
        
        Room{spheres: btmap}
    }

    pub fn shape_list<'s> (&'s self) -> ShapeList<'s> {
        let mut l = ShapeList::new();
        for (_, s) in self.spheres.iter() {
            l.add_shape(s as &Surface);
        }
        l
    }
}

pub fn setup_camera (ctrl: &mut FPSCameraController) {
    //cam(Vec(50,52,295.6), Vec(0,-0.042612,-1).norm()); // cam pos, dir
    ctrl.camera_mut().set_pos(&Point3f::new(-25.0, -25.0, 49.0));
    ctrl.camera_mut().yaw_add((-30.0 as Real).to_radians());
    ctrl.set_move_speed(50.0);
    ctrl.set_mouse_sensitivity(0.20);
}