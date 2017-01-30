use super::na;
// use na::*;
use na::{Inverse, Isometry3, Matrix4, Norm, Point3, Rotate, Rotation, ToHomogeneous, Vector3};

use consts;



pub trait RenderCamera {
    fn view_matrix(&self) -> Matrix4<f32>;
    fn proj_matrix(&self) -> Matrix4<f32>;


    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn aspect(&self) -> f32;
    fn znear(&self) -> f32;
    fn zfar(&self) -> f32;
    fn fovx(&self) -> f32;
}

pub struct FPSCamera {
    width: u32,
    height: u32,
    fovx: f32,
    znear: f32,
    zfar: f32,

    // trfm: Matrix4<f32>,
    trfm: Isometry3<f32>,
}

impl FPSCamera {
    pub fn new(width: u32, height: u32, fovx: f32, znear: f32, zfar: f32) -> FPSCamera {
        FPSCamera {
            width: width,
            height: height,
            fovx: fovx,
            znear: znear,
            zfar: zfar,
            trfm: na::one(),
        }
    }

    pub fn look_at(&mut self, target: &Point3<f32>) {
        self.trfm = Isometry3::look_at_rh(self.trfm.translation.as_point(),
                                          target,
                                          &na::Vector3::from(&consts::_3d::up_vec));
    }

    pub fn transform(&self) -> &Isometry3<f32> {
        &self.trfm
    }

    pub fn right(&self) -> Vector3<f32> {
        self.trfm.rotate(&Vector3::from(&consts::_3d::right_vec))
    }

    pub fn up(&self) -> Vector3<f32> {
        self.trfm.rotate(&Vector3::from(&consts::_3d::up_vec))
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.trfm.rotate(&Vector3::from(&consts::_3d::forward_vec))
    }

    pub fn pos(&self) -> &Point3<f32> {
        self.trfm.translation.as_point()
    }

    pub fn pos_add(&mut self, delta_pos: &Vector3<f32>) -> &mut FPSCamera {
        self.trfm.translation += *delta_pos;
        self
    }
    pub fn set_pos(&mut self, pos: &Point3<f32>) -> &mut FPSCamera {
        self.trfm.translation = *pos.as_vector();
        self
    }

    // pub fn yaw(&self) -> f32 {     }
    // pub fn pitch(&self, angle: f32) -> f32 {   }
    // pub fn roll(&self) -> f32;
    //
    //
    pub fn yaw_add(&mut self, angle: f32) -> &mut FPSCamera {
        let cam_right = self.right();
        self.trfm.rotation.append_rotation_mut(&(cam_right * angle));
        self
    }
    pub fn pitch_add(&mut self, angle: f32) -> &mut FPSCamera {
        // let cam_up = self.trfm.rotate(&Vector3::from(&consts::_3d::up_vec)).normalize();
        let up = Vector3::from(&consts::_3d::up_vec);
        self.trfm.rotation.append_rotation_mut(&(up * angle));
        self
    }
    // pub fn roll_add (&mut self, angle: f32) -> &mut FPSCamera;
    //
}


impl RenderCamera for FPSCamera {
    fn view_matrix(&self) -> Matrix4<f32> {
        self.trfm.inverse().unwrap_or(na::one()).to_homogeneous()
    }
    fn proj_matrix(&self) -> Matrix4<f32> {
        na::one()
    }


    fn height(&self) -> u32 {
        self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    fn znear(&self) -> f32 {
        self.znear
    }
    fn zfar(&self) -> f32 {
        self.zfar
    }
    fn fovx(&self) -> f32 {
        self.fovx
    }
}
