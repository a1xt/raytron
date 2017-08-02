use rtcore::RenderCamera;
use rtcore::math;
use rtcore::math::{Inverse, Rotate, Rotation, ToHomogeneous};
use rtcore::math::{Isometry3, Matrix4, PerspectiveMatrix3, Rotation3, Vector3};
use rtcore::math::{Point3f, Real, Vector3f};
use rtcore::utils::consts;

#[derive(Debug, Clone, Copy)]
pub struct FPSCamera {
    width: u32,
    height: u32,
    fovy: Real,
    znear: Real,
    zfar: Real,
    trfm: Isometry3<Real>,
}

impl FPSCamera {
    pub fn new(width: u32, height: u32, fovy: Real, znear: Real, zfar: Real) -> FPSCamera {
        FPSCamera {
            width: width,
            height: height,
            fovy: fovy,
            znear: znear,
            zfar: zfar,
            trfm: math::one(),
        }
    }

    pub fn look_at(&mut self, target: &Point3f) {
        self.trfm.rotation =
            Rotation3::look_at_lh(&(*target - *self.pos()), &Vector3f::from(&consts::UP_VEC));
    }

    pub fn transform(&self) -> &Isometry3<Real> {
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

    pub fn yaw_add(&mut self, angle: Real) -> &mut FPSCamera {
        let up = Vector3::from(&consts::UP_VEC);
        self.trfm.rotation.append_rotation_mut(&(up * angle));
        self
    }

    pub fn pitch_add(&mut self, angle: Real) -> &mut FPSCamera {
        let cam_right = self.right();
        self.trfm.rotation.append_rotation_mut(&(cam_right * angle));
        self
    }

    pub fn set_rotation(&mut self, yaw: Real, pitch: Real) -> &mut FPSCamera {
        self.trfm.rotation = Rotation3::from_euler_angles(0.0, yaw, pitch);
        self
    }
}

impl RenderCamera for FPSCamera {
    fn view_matrix(&self) -> Matrix4<Real> {
        self.trfm
            .inverse()
            .unwrap_or_else(math::one)
            .to_homogeneous()
    }
    fn proj_matrix(&self) -> Matrix4<Real> {
        PerspectiveMatrix3::new(
            self.width as Real / self.height as Real,
            self.fovy,
            self.znear,
            self.zfar,
        ).to_matrix()
    }

    fn height(&self) -> u32 {
        self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn aspect(&self) -> Real {
        self.width as Real / self.height as Real
    }
    fn znear(&self) -> Real {
        self.znear
    }
    fn zfar(&self) -> Real {
        self.zfar
    }
    fn fovy(&self) -> Real {
        self.fovy
    }
    fn fovx(&self) -> Real {
        let ratio = self.width as Real / self.height as Real;
        2.0 * (ratio * (0.5 * self.fovy).tan()).atan()
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
