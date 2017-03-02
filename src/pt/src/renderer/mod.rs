pub mod pathtracer;
pub mod dbgraycaster;

pub use self::pathtracer::PathTracer;
pub use self::dbgraycaster::DbgRayCaster;

use traits::{RenderCamera, SceneHolder};
use math::{Ray3f, Matrix4f, Vector3f, Point3f, Coord};
use math::{self, Norm};
use {SurfacePoint, Color, RenderSettings, Image};
use image::ImageBuffer;
use color;
use rand::{self, Closed01};

use std::{self, thread};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::ops::DerefMut;

use scoped_threadpool::{Pool};

use self::inner::RendererHelper;

mod inner {
    use traits::{RenderCamera, SceneHolder};
    use math::{self, Ray3f, Point3f, Vector3f, Coord, Norm};
    use {Image, RenderSettings, Color};
    use rand::{self, Closed01};
    use color;
    
    pub trait RendererHelper<S: SceneHolder, C: RenderCamera> {

        fn trace_path(&self, scene: &S, initial_ray: &Ray3f, setup: &RenderSettings) -> Color;
        
        fn get_ray(&self, camera: &C, x: u32, y: u32) -> Ray3f;

        fn render_job<'a>(&self, scene: &S, camera: &C, setup: &RenderSettings, img_rect: ((u32, u32), (u32, u32))) -> Vec<Color> {
            let ((x0, y0), (img_w, img_h))  = img_rect;
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

        fn add_to_pixel(&self, c: &Color, pnum: f32, x: u32, y: u32, out_image: &mut Image) {
            let mut pixel = out_image.get_pixel(x, y).clone();
            pixel = color::mul_s(&pixel, pnum - 1.0);
            pixel = color::sum(&pixel, &c);
            pixel = color::mul_s(&pixel, 1.0 / pnum);
            out_image.put_pixel(x, y, pixel);
        }
        
    }

    pub struct CameraRayGenerator {
        origin: Point3f,
        up: Vector3f,
        forward: Vector3f,
        right: Vector3f,

        x0: Coord,
        y0: Coord,
        dx: Coord,
        dy: Coord,
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
        pub fn with_camera <C: RenderCamera> (camera: &C) -> CameraRayGenerator {

            let origin = camera.pos();
            let up = camera.up_vec().normalize();
            let forward = camera.forward_vec().normalize();
            let right = camera.right_vec().normalize();

            let ratio = camera.height() as Coord / camera.width() as Coord;
            let x = 2.0 * (0.5 * camera.fovx()).tan();
            let y = ratio * x;
            let dx = x / (camera.width() as Coord);
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

            let Closed01(rnd_x) = rand::random::<Closed01<Coord>>();
            let Closed01(rnd_y) = rand::random::<Closed01<Coord>>();      

            let rx = self.x0 + self.dx * (x as Coord) + self.dx * (rnd_x - 0.5);
            let ry = self.y0 - self.dy * (y as Coord) + self.dy * (rnd_y - 0.5);

            let ray_dir = self.forward + self.right * rx + self.up * ry;
            let ray = Ray3f::new(&self.origin, &ray_dir.normalize());

            ray
        }
    }
}

pub trait Renderer<S: SceneHolder + Sync, C: RenderCamera + Sync> : RendererHelper<S, C> + Sync {

    fn pre_render(&mut self, scene: &S, camera: &C, setup: &RenderSettings);

    fn render_scene(&mut self, scene: &S, camera: &C, setup: &RenderSettings, out_image: &mut Image) {
        self.pre_render(scene, camera, setup);
        for p in 0..setup.samples_per_pixel {
            self.render_pass(scene, camera, setup, p, out_image);
        }
    }

    fn render_scene_threaded(&mut self, scene: &S, camera: &C, setup: &RenderSettings, out_image: &mut Image) {
        self.pre_render(scene, camera, setup);
        for p in 0..setup.samples_per_pixel {
            self.render_pass_threaded(scene, camera, setup, p, out_image);
        }
    }

    fn render_pass(&self, scene: &S, camera: &C, setup: &RenderSettings, pass_num: u32, out_image: &mut Image) {
        let pnum: f32 = if pass_num == 0 {
            1.0
        } else {
            pass_num as f32
        };

        for j in 0..camera.height() {
            for i in 0..camera.width() {
                let ray = self.get_ray(camera, i, j);
                let c = self.trace_path(scene, &ray, setup);
                self.add_to_pixel(&c, pnum, i, j, out_image);
            }
        }
    }

    fn render_pass_threaded(&self, scene: &S, camera: &C, setup: &RenderSettings, pass_num: u32, out_image: &mut Image) {

        let pnum: f32 = if pass_num == 0 {
            1.0
        } else {
            pass_num as f32
        }; 
        
        let (block_w, block_h) = setup.render_block;
        let blocks_num = (camera.width() / block_w) * (camera.height() / block_h);
        // let mut res_blocks: Vec<Vec<Color>> = Vec::with_capacity(blocks_num as usize);

        // for _ in 0..blocks_num {
        //     res_blocks.push(Vec::new());
        // }

        let out_img = Arc::new(Mutex::new(out_image));
        
        let mut pool = Pool::new(setup.threads_num);
        pool.scoped(|scope| { 

            let mut offset_x = 0;
            let mut offset_y = 0;

            //for b in &mut res_blocks {
            for _ in 0..blocks_num {

                let tmp_out_img = out_img.clone();

                scope.execute(move || {
                    let block = self.render_job(scene, camera, setup, ((offset_x, offset_y),(block_w, block_h)));
                    //*b = block;
                    let mut img = tmp_out_img.lock().unwrap();
                    for j in 0..block_h {
                        for i in 0..block_w {
                            let c = block[(j * block_w + i) as usize];
                            self.add_to_pixel(&c, pnum, offset_x + i, offset_y + j, img.deref_mut());
                        }
                    }
                });

                offset_x += block_w;
                if offset_x >= camera.width() {
                    offset_x = 0;
                    offset_y += block_h;
                }
            }

            scope.join_all();

        });

        // let mut block_ix = 0;
        // for offset_y in (0..camera.height()).step_by(block_h) {
        //     for offset_x in (0..camera.width()).step_by(block_w) {    
                
        //         for j in 0..block_h {
        //             for i in 0..block_w {
        //                 let res = &res_blocks[block_ix];
        //                 let c = res[(j * block_w + i) as usize];
        //                 self.add_to_pixel(&c, pnum, offset_x + i, offset_y + j, out_image);                        
        //             }
        //         }
        //         block_ix += 1;
        //     }
        // }        
 
    }
    
}

