#![feature(box_syntax)]

pub mod common;
use common::*;

use scenes::spheres;
use pt::traits::{SceneHolder};


pub struct Spheres {
    room: spheres::Room,
}

impl AppState for Spheres {
    fn new() -> Self {
        Self {
            room: spheres::Room::new(),
        }
    }

    fn init(&self) -> ExampleAppBuilder {
        let builder = ExampleAppBuilder::new().size(400, 300);
        builder
    }

    fn init_camera(&self, camera: &mut FPSCameraController) {
        spheres::setup_camera(camera);
    }

    fn create_scene<'s>(&'s self) -> Box<SceneHolder + 's> {
        box self.room.shape_list()
    }


    // fn update(&mut self) { }
    // fn need_update(&self) -> bool { false }
    // fn post_process(&self, &mut Image) { }
    // fn init_camera(&self, &mut FPSCameraController) { }
    // //fn update_camera(&self, &mut CameraController) { }

    // fn create_renderer<'s>(&'s self) -> (Box<Renderer<SceneHolder + 's> + 's>, RenderSettings) {
    //     let pt_render_chunk = (80, 60);
    //     let rdr_setup = RenderSettings::new(128, 10).with_threads(4, pt_render_chunk);       
    //     let rdr = box PathTracer::new(&rdr_setup).with_direct_illumination(0.75, 0.25);
    //     (rdr, rdr_setup)
    // }

}

fn main() {
    let mut state = Spheres::new();
    ExampleApp::launch(&mut state);
}