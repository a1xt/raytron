pub mod pathtracer;
pub mod dbgraycaster;

pub use self::dbgraycaster::DbgRayCaster;

use self::inner::RendererHelper;
pub use self::pathtracer::PathTracer;
use {RenderSettings, Color};

use scoped_threadpool::Pool;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use traits::{RenderCamera, SceneHandler, TexView};

mod inner {
    use {RenderSettings, Color};
    use math::{self, Ray3f, Point3f, Vector3f, Real, Norm};
    use rand::{self, Closed01};
    use traits::{RenderCamera, SceneHandler, TexView};

    pub trait RendererHelper<S, C>: Sync
    where
        S: SceneHandler + ?Sized,
        C: RenderCamera + ?Sized,
    {
        fn trace_path(&self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color;

        fn get_ray(&self, camera: &C, x: u32, y: u32) -> Ray3f;

        fn render_job(
            &self,
            scene: &S,
            camera: &C,
            setup: &RenderSettings,
            img_rect: ((u32, u32), (u32, u32)),
        ) -> Vec<Color> {
            let ((x0, y0), (img_w, img_h)) = img_rect;
            let mut result = Vec::with_capacity((img_w * img_h) as usize);
            for y in 0..img_h {
                for x in 0..img_w {
                    let ray = self.get_ray(camera, x + x0, y + y0);
                    let color = self.trace_path(scene, &ray, setup);
                    result.push(color);
                    //result[(y * img_w + x) as usize] = color;
                }
            }
            result

        }

        fn add_to_pixel(
            &self,
            c: &Color,
            pnum: f32,
            x: u32,
            y: u32,
            out_image: &mut TexView<Color>,
        ) {
            let mut pixel = out_image.pixel(x as usize, y as usize);
            pixel *= pnum - 1.0;
            pixel += *c;
            pixel *= 1.0 / pnum;
            out_image.set_pixel(x as usize, y as usize, pixel);
        }
    }

    pub struct CameraRayGenerator {
        origin: Point3f,
        up: Vector3f,
        forward: Vector3f,
        right: Vector3f,

        x0: Real,
        y0: Real,
        dx: Real,
        dy: Real,
    }

    impl CameraRayGenerator {
        pub fn new() -> CameraRayGenerator {
            CameraRayGenerator {
                origin: math::origin(),
                up: math::zero(),
                forward: math::zero(),
                right: math::zero(),

                x0: 0.0,
                y0: 0.0,
                dx: 0.0,
                dy: 0.0,
            }
        }

        pub fn with_camera<C: RenderCamera + ?Sized>(camera: &C) -> CameraRayGenerator {

            let origin = camera.pos();
            let up = camera.up_vec().normalize();
            let forward = camera.forward_vec().normalize();
            let right = camera.right_vec().normalize();

            let ratio = camera.height() as Real / camera.width() as Real;
            let x = 2.0 * (0.5 * camera.fovx()).tan();
            let y = ratio * x;
            let dx = x / (camera.width() as Real);
            let dy = dx;

            let x0 = (-0.5) * x + dx * 0.5;
            let y0 = 0.5 * y - dy * 0.5;


            CameraRayGenerator {
                origin: origin,
                up: up,
                forward: forward,
                right: right,

                x0: x0,
                y0: y0,
                dx: dx,
                dy: dy,
            }
        }

        pub fn get_ray(&self, x: u32, y: u32) -> Ray3f {

            let Closed01(rnd_x) = rand::random::<Closed01<Real>>();
            let Closed01(rnd_y) = rand::random::<Closed01<Real>>();

            let rx = self.x0 + self.dx * (x as Real) + self.dx * (rnd_x - 0.5);
            let ry = self.y0 - self.dy * (y as Real) + self.dy * (rnd_y - 0.5);
            let ray_dir = self.forward + self.right * rx + self.up * ry;

            Ray3f::new(&self.origin, &ray_dir.normalize())
        }
    }
}

pub trait Renderer<S = SceneHandler, C = RenderCamera>
    : RendererHelper<S, C> + Sync
where
    S: SceneHandler + ?Sized,
    C: RenderCamera + ?Sized,
{
    fn pre_render(&mut self, scene: &S, camera: &C, setup: &RenderSettings);

    fn render_scene(
        &mut self,
        scene: &S,
        camera: &C,
        setup: &RenderSettings,
        out_image: &mut TexView<Color>,
    ) {
        self.pre_render(scene, camera, setup);
        for p in 0..setup.samples_per_pixel {
            self.render_pass(scene, camera, setup, p, out_image);
        }
    }

    fn render_scene_threads(
        &mut self,
        scene: &S,
        camera: &C,
        setup: &RenderSettings,
        out_image: &mut TexView<Color>,
    ) {
        self.pre_render(scene, camera, setup);
        for p in 0..setup.samples_per_pixel {
            self.render_pass_threads(scene, camera, setup, p, out_image);
        }
    }

    fn render_pass(
        &self,
        scene: &S,
        camera: &C,
        setup: &RenderSettings,
        pass_num: u32,
        out_image: &mut TexView<Color>,
    ) {
        let pnum: f32 = if pass_num == 0 { 1.0 } else { pass_num as f32 };

        for j in 0..camera.height() {
            for i in 0..camera.width() {
                let ray = self.get_ray(camera, i, j);
                let c = self.trace_path(scene, &ray, setup);
                self.add_to_pixel(&c, pnum, i, j, out_image);
            }
        }
    }

    fn render_pass_threads(
        &self,
        scene: &S,
        camera: &C,
        setup: &RenderSettings,
        pass_num: u32,
        out_image: &mut TexView<Color>,
    ) {

        let pnum: f32 = if pass_num == 0 { 1.0 } else { pass_num as f32 };

        let (chunk_w, chunk_h) = setup.render_chunk;
        let chunks_num = (camera.width() / chunk_w) * (camera.height() / chunk_h);
        // let mut res_chunks: Vec<Vec<Color>> = Vec::with_capacity(chunks_num as usize);

        // for _ in 0..chunks_num {
        //     res_chunks.push(Vec::new());
        // }

        let out_img = Arc::new(Mutex::new(out_image));

        let mut pool = Pool::new(setup.threads_num);
        pool.scoped(|scope| {

            let mut offset_x = 0;
            let mut offset_y = 0;

            //for b in &mut res_chunks {
            for _ in 0..chunks_num {

                let tmp_out_img = out_img.clone();

                scope.execute(move || {
                    let chunk = self.render_job(
                        scene,
                        camera,
                        setup,
                        ((offset_x, offset_y), (chunk_w, chunk_h)),
                    );
                    //*b = chunk;
                    let mut img = tmp_out_img.lock().unwrap();
                    for j in 0..chunk_h {
                        for i in 0..chunk_w {
                            let c = chunk[(j * chunk_w + i) as usize];
                            self.add_to_pixel(
                                &c,
                                pnum,
                                offset_x + i,
                                offset_y + j,
                                *img.deref_mut(),
                            );
                        }
                    }
                });

                offset_x += chunk_w;
                if offset_x >= camera.width() {
                    offset_x = 0;
                    offset_y += chunk_h;
                }
            }

            scope.join_all();

        });

        // let mut chunk_ix = 0;
        // for offset_y in (0..camera.height()).step_by(chunk_h) {
        //     for offset_x in (0..camera.width()).step_by(chunk_w) {

        //         for j in 0..chunk_h {
        //             for i in 0..chunk_w {
        //                 let res = &res_chunks[chunk_ix];
        //                 let c = res[(j * chunk_w + i) as usize];
        //                 self.add_to_pixel(&c, pnum, offset_x + i, offset_y + j, out_image);
        //             }
        //         }
        //         chunk_ix += 1;
        //     }
        // }

    }
}
