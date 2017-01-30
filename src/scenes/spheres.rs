use pt::sceneholder::{ShapeList};
use pt::material::{Diffuse, Phong};
use pt::{Sphere, Color};
use pt::math;
use pt::math::{Vector3f, Point3f};
use pt::color::WHITE;
use camera_controller::{FPSCameraController};


pub fn create_scene<'s> () -> ShapeList<'s> {
    let mut shp_list= ShapeList::new();  

   //Left
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(-1.0e5 - 50.0, 0.0, 0.0),
            1.0e5,
            Box::new(Diffuse::new(Color{data: [0.75, 0.25, 0.25f32, 1.0]}, None)),
        ),
        false
    );
   //Right
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(1.0e3 + 50.0, 0.0, 0.0),
            1.0e3, ////
            //Box::new(Diffuse::new(Color{data: [0.25, 0.25, 0.75f32, 1.0]}, None)),
            Box::new(Phong::new(Color{data: [0.25, 0.25, 0.75f32, 1.0]}, 0.1, 0.9, 20)),
        ),
        false
    );
   //Back
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(0.0, 0.0, 1.0e5 + 50.0),
            1.0e5,
            //Box::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
            Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.1, 0.9, 100000))
        ),
        false
    );
   //Front
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(0.0, 0.0, -1.0e3 - 50.0),
            1.0e3,///////
            Box::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
            //Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.1, 0.9, 100000))
        ),
        false
    );
   //Bottom
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(0.0, -1.0e5 - 50.0, 0.0),
            1.0e5,
            //Box::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
            Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.2, 0.8, 5)),
        ),
        false
    );
   //Top
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(0., 1.0e5 + 50.0, 0.),
            1.0e5,
            Box::new(Diffuse::new(Color{data: [0.75, 0.75, 0.75f32, 1.0]}, None)),
        ),
        false
    );
   //Sphere 1
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(-20., -35.0, -20.),
            15.0,
            //Box::new(Diffuse::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, None)),
            Box::new(Diffuse::new(Color{data: [0.9, 0.9, 0.9f32, 1.0]}, Some(Color{data: [0.0, 0.2, 0.0, 1.0]}))),
        ),
        false
    );
   //Sphere 2
   shp_list.add_shape(
        Sphere::new(
            Point3f::new(15.0, -35.0, 5.0),
            15.,
            //Box::new(Diffuse::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, None)),
            Box::new(Phong::new(Color{data: [0.999, 0.999, 0.999f32, 1.0]}, 0.3, 0.7, 100000)),
        ),
        false
    );
   //Light source
    shp_list.add_shape(
        Sphere::new(
            Point3f::new(0.0, 60.0, -10.0),
            15.0,
            Box::new(Diffuse::new(Color{data: [1.0, 1.0, 1.0f32, 1.0]}, Some(Color{data: [12.0, 12.0, 12.0f32, 1.0]}))),
        ),
        true
    );

    shp_list
}

pub fn setup_camera (ctrl: &mut FPSCameraController) {
    //cam(Vec(50,52,295.6), Vec(0,-0.042612,-1).norm()); // cam pos, dir
    ctrl.camera_mut().set_pos(&Point3f::new(0.0, 0.0, -100.0));
    ctrl.camera_mut().look_at(&Point3f::new(0.0, 0.0, 0.0));
    ctrl.set_move_speed(100.0);
    ctrl.set_mouse_sensitivity(0.25);
}