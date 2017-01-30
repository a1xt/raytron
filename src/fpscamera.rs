use pt::{RenderCamera};
use pt::utils::consts;
use pt::math;
use pt::math::{Matrix4, Point3, Vector3, Isometry3, Rotation3, PerspectiveMatrix3};
use pt::math::{Vector3f, Point3f, Coord};
use pt::math::{ToHomogeneous, Rotation, Rotate, Inverse};

#[derive(Debug, Clone, Copy)]
pub struct FPSCamera {
    width: u32,
    height: u32,
    fovx: Coord,
    znear: Coord,
    zfar: Coord,
    trfm: Isometry3<Coord>,
}

impl FPSCamera {
    pub fn new(width: u32, height: u32, fovx: Coord, znear: Coord, zfar: Coord) -> FPSCamera {
        FPSCamera {
            width: width,
            height: height,
            fovx: fovx,
            znear: znear,
            zfar: zfar,
            trfm: math::one(),
        }
    }

    pub fn look_at(&mut self, target: &Point3f) {
        self.trfm = Isometry3::look_at_rh(self.trfm.translation.as_point(),
                                          target,
                                          &math::Vector3::from(&consts::UP_VEC));
    }

    pub fn transform(&self) -> &Isometry3<Coord> {
        &self.trfm
    }

    pub fn right(&self) -> Vector3f {
        self.trfm.rotate(&Vector3::from(&consts::RIGHT_VEC))
    }

    pub fn up(&self) -> Vector3f {
        self.trfm.rotate(&Vector3::from(&consts::UP_VEC))
    }

    pub fn forward(&self) -> Vector3f {
        self.trfm.rotate(&Vector3::from(&consts::FORWARD_VEC))
    }

    pub fn pos(&self) -> &Point3f {
        self.trfm.translation.as_point()
    }

    pub fn pos_add(&mut self, delta_pos: &Vector3f) -> &mut FPSCamera {
        self.trfm.translation += *delta_pos;
        self
    }

    pub fn set_pos(&mut self, pos: &Point3f) -> &mut FPSCamera {
        self.trfm.translation = *pos.as_vector();
        self
    }

    pub fn yaw_add(&mut self, angle: Coord) -> &mut FPSCamera {
        let cam_right = self.right();
        self.trfm.rotation.append_rotation_mut(&(cam_right * angle));
        self
    }

    pub fn pitch_add(&mut self, angle: Coord) -> &mut FPSCamera {
        let up = Vector3::from(&consts::UP_VEC);
        self.trfm.rotation.append_rotation_mut(&(up * angle));
        self
    }

    pub fn set_rotation(&mut self, yaw: Coord, pitch: Coord) -> &mut FPSCamera {
        self.trfm.rotation = Rotation3::from_euler_angles(0.0, yaw, pitch);
        self
    }
}

impl RenderCamera for FPSCamera {
    fn view_matrix(&self) -> Matrix4<Coord> {
        self.trfm
            .inverse()
            .unwrap_or(math::one())
            .to_homogeneous()
    }
    fn proj_matrix(&self) -> Matrix4<Coord> {
        PerspectiveMatrix3::new(self.width as Coord / self.height as Coord,
                                self.fovx,
                                self.znear,
                                self.zfar)
            .to_matrix()
    }

    fn height(&self) -> u32 {
        self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn aspect(&self) -> Coord {
        self.width as Coord / self.height as Coord
    }
    fn znear(&self) -> Coord {
        self.znear
    }
    fn zfar(&self) -> Coord {
        self.zfar
    }
    fn fovx(&self) -> Coord {
        self.fovx
    }

    fn pos(&self) -> Point3f {
        *self.pos()
    }

    fn up_vec(&self) -> Vector3f {
        self.up()
    }
    fn forward_vec(&self) -> Vector3f {
        self.forward()
    }
    fn right_vec(&self) -> Vector3f {
        self.right()
    }
}
