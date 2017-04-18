use math::{Ray3f, Matrix4f, Vector3f, Point3f, Real, Dot, Norm};
use super::{SurfacePoint, Color};

pub use renderer::Renderer;
pub use sceneholder::SceneHolder;
pub use bsdf::Bsdf;

pub trait RenderCamera {
    fn view_matrix(&self) -> Matrix4f;
    fn proj_matrix(&self) -> Matrix4f;

    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn aspect(&self) -> Real;
    fn znear(&self) -> Real;
    fn zfar(&self) -> Real;
    fn fovx(&self) -> Real;

    fn pos(&self) -> Point3f;
    fn up_vec(&self) -> Vector3f;
    fn forward_vec(&self) -> Vector3f;
    fn right_vec(&self) -> Vector3f;
}

pub trait Surface : Sync {
    /// return (t, sp)
    fn intersection (&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)>;

    fn is_emitter(&self) -> bool;

    /// ∫ Le dA
    fn total_emittance(&self) -> Option<Color>;

    fn area (&self) -> Real;
    fn normal_at(&self, pos: &Point3f) -> Vector3f;

    // return (random point, pdf)
    // fn sample_surface(&self, view_point: &Point3f) -> (SurfacePoint, Real);
    // fn pdf(&self,  point_at_surface: &Point3f, view_point: &Point3f) -> Real;
    
    fn sample_surface_p(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let (sp, pdf_d) = self.sample_surface_d(view_point);
        let view_dir = *view_point.0 - sp.position;
        let r2 = view_dir.norm_squared();
        let cos_theta_l = sp.normal.dot(&view_dir.normalize());
        let pdf_p = pdf_d * (cos_theta_l / r2);
        (sp, pdf_p)
    }

    fn sample_surface_d(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let (sp, pdf_p) = self.sample_surface_p(view_point);
        let view_dir = *view_point.0 - sp.position;
        let r2 = view_dir.norm_squared();
        let cos_theta_l = sp.normal.dot(&view_dir.normalize());
        let pdf_d = pdf_p * (r2 / cos_theta_l);
        (sp, pdf_d)
    }

    fn sample_surface_d_proj(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let (sp, pdf_d) = self.sample_surface_d(view_point);
        let view_dir_inv = (sp.position - *view_point.0).normalize();
        let cos_theta = view_point.1.dot(&view_dir_inv);
        let pdf_d_proj = pdf_d / cos_theta;
        (sp, pdf_d_proj)
    }

    fn pdf_p(&self,  point_at_surface: (&Point3f, &Vector3f), view_point: (&Point3f, &Vector3f)) -> Real {
        let pdf_d = self.pdf_d(point_at_surface, view_point);
        let view_dir = *view_point.0 - *point_at_surface.0;
        let r2 = view_dir.norm_squared();
        let cos_theta_l = point_at_surface.1.dot(&view_dir.normalize());
        let pdf_p = pdf_d * (cos_theta_l / r2);
        pdf_p
    }

    fn pdf_d(&self,  point_at_surface: (&Point3f, &Vector3f), view_point: (&Point3f, &Vector3f)) -> Real {
        let pdf_p = self.pdf_p(point_at_surface, view_point);
        let view_dir = *view_point.0 - *point_at_surface.0;
        let r2 = view_dir.norm_squared();
        let cos_theta_l = point_at_surface.1.dot(&view_dir.normalize());
        let pdf_d = pdf_p * (r2 / cos_theta_l);
        pdf_d
    }

    fn pdf_d_proj(&self,  point_at_surface: (&Point3f, &Vector3f), view_point: (&Point3f, &Vector3f)) -> Real {
        let pdf_d = self.pdf_d(point_at_surface, view_point);
        let view_dir_inv = (*point_at_surface.0 - *view_point.0).normalize();
        let cos_theta = view_point.1.dot(&view_dir_inv);
        let pdf_d_proj = pdf_d / cos_theta;
        pdf_d_proj
    }
}