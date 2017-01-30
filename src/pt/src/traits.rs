use math::{Ray3f, Matrix4f, Vector3f, Point3f, Coord};
use math::{Norm};
use math;
use super::{SurfacePoint, Color, RenderSettings, Image};
use std::f32;
use image::ImageBuffer;
use color;
use rand::{Closed01};
use rand;


pub trait RenderCamera {
    fn view_matrix(&self) -> Matrix4f;
    fn proj_matrix(&self) -> Matrix4f;

    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn aspect(&self) -> Coord;
    fn znear(&self) -> Coord;
    fn zfar(&self) -> Coord;
    fn fovx(&self) -> Coord;

    fn pos(&self) -> Point3f;
    fn up_vec(&self) -> Vector3f;
    fn forward_vec(&self) -> Vector3f;
    fn right_vec(&self) -> Vector3f;
}

pub trait Surface {
    /// return (t, sp)
    fn intersection (&self, ray: &Ray3f) -> Option<(Coord, SurfacePoint)>;
    fn random_point (&self) -> SurfacePoint;
}

pub trait SceneHolder {
    fn intersection_with_scene(&self, ray: &Ray3f) -> Option<SurfacePoint>;
    fn random_light_source<'s>(&'s self) -> Option<&'s Surface>;
    //fn ligth_sources();
}

pub trait Renderer<S: SceneHolder, C: RenderCamera> {    
    fn render_scene(&mut self, scene: &S, camera: &C, setup: &RenderSettings, out_image: &mut Image) {

        for p in 0..setup.samples_per_pixel {
            self.render_pass(scene, camera, setup, p, out_image);
            println!("pass num: {}", p);
        }

        
    }

    fn render_pass(&mut self, scene: &S, camera: &C, setup: &RenderSettings, pass_num: u32, out_image: &mut Image) {

        let pnum: f32 = if pass_num == 0 { 1.0 } else { pass_num as f32 };

        let ratio = camera.height() as Coord / camera.width() as Coord;
        let x = 2.0 * (0.5 * camera.fovx()).tan() as Coord;
        let y = ratio * x;
        let dx = x / camera.width() as Coord;
        let dy = dx;

        let x0 = (-0.5) * x + dx * 0.5;
        let y0 = 0.5 * y - dy * 0.5;

        let origin = camera.pos();
        let up = camera.up_vec().normalize();
        let forward = camera.forward_vec().normalize();
        let right = camera.right_vec().normalize();

        for i in 0..camera.height() {
            for j in 0..camera.width() {      

                let Closed01(rndx) = rand::random::<Closed01<Coord>>();
                let Closed01(rndy) = rand::random::<Closed01<Coord>>();      

                let rx = x0 + dx * (j as Coord) + dx * (rndx - 0.5);
                let ry = y0 - dy * (i as Coord) + dy * (rndy - 0.5);

                let rdir = forward + right * rx + up * ry;
                let ray = Ray3f::new(origin, rdir.normalize());

                //let mut pixel = color::BLACK;
                //let mut pixel = Color {data: [4.0, 0.0, 0.0, 1.0]};

                //for _ in 0..setup.samples_per_pixel {
                let c = self.trace_path(scene, &ray, setup);
                //println!("c: {:?}", c);
               //pixel = color::sum(&pixel,&c);
                    
                //}
                //pixel = color::mul_s(&pixel, 1.0 / setup.samples_per_pixel as f32);
                let mut pixel = out_image.get_pixel(j, i).clone();
                pixel = color::mul_s(&pixel, pnum - 1.0);
                pixel = color::sum(&pixel, &c);
                pixel = color::mul_s(&pixel, 1.0 / pnum);
                out_image.put_pixel(j, i, pixel);
            }
            //println!("row: {}", i);
        }
    }

    fn trace_path(&mut self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color;
}

pub trait Material {

    // fn emission(&self) -> Option<f32>;
    // fn color(&self) -> Color;
    // fn reflectance(&self) -> f32;
    // fn reflect_ray<F: BaseFloat>(&self, ray: Ray<F>, normal: Vector3<F>);

    fn emission(&self) -> Option<Color>;
    fn reflectance(&self, ray: &Vector3f, reflected_ray: &Vector3f, normal: &Vector3f) -> Color;
    fn reflect_ray(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> Ray3f;

    /// return (reflected ray, reflectance)
    fn brdf(&self, ray_dir: &Vector3f, surface_point: &Point3f, surface_normal: &Vector3f) -> (Ray3f, Color);

}