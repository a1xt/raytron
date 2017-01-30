use glutin::{Event, ElementState, VirtualKeyCode, MouseButton};
use std::time;
use std::time::Instant;
use std::ops::{FnMut};
use pt::math;
use pt::math::{Vector3, ApproxEq, Norm, Coord};
use pt::utils::consts;
use pt::RenderCamera;
use fpscamera::{FPSCamera};


pub trait CameraController {
    fn on_frame<'b, 'a, 'c, I: Iterator<Item = &'a Event>>(&mut self,
                                                       event_iter: I,
                                                       set_cursor_pos: &'b mut FnMut(i32, i32),
                                                       cur_locker: &'c mut FnMut(bool));
}

#[derive(Debug, Clone, Copy)]
pub struct FPSCameraController {
    cam: FPSCamera,
    mouse_sens: Coord,
    move_speed: Coord,

    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
    cursor_locked: bool,

    last_tp: time::Instant,
}

impl FPSCameraController {
    pub fn new(cam: FPSCamera, mouse_sensitivity: Coord, move_speed: Coord) -> FPSCameraController {
        FPSCameraController {
            cam: cam,
            mouse_sens: mouse_sensitivity,
            move_speed: move_speed,
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
            cursor_locked: false,

            last_tp: Instant::now(),
        }
    }

    pub fn mouse_sensitivity(&self) -> Coord {
        self.mouse_sens
    }

    pub fn set_mouse_sensitivity(&mut self, val: Coord) -> &mut FPSCameraController {
        self.mouse_sens = val;
        self
    }

    pub fn move_speed(&self) -> Coord {
        self.move_speed
    }

    pub fn set_move_speed(&mut self, speed: Coord) -> &mut FPSCameraController {
        self.move_speed = speed;
        self
    }

    pub fn camera(&self) -> &FPSCamera {
        &self.cam
    }

    pub fn camera_mut(&mut self) -> &mut FPSCamera {
        &mut self.cam
    }

    fn apply_move(&mut self, delta_time: Coord) {
        let mut mdir: Vector3<Coord> = math::zero();
        if self.move_forward {
            mdir += self.cam.forward();
        }
        if self.move_backward {
            mdir -= self.cam.forward();
        }
        if self.move_right {
            mdir += self.cam.right();
        }
        if self.move_left {
            mdir -= self.cam.right();
        }
        if self.move_up {
            mdir += Vector3::from(&consts::UP_VEC);
        }
        if self.move_down {
            mdir -= Vector3::from(&consts::UP_VEC);
        }

        if !mdir.approx_eq(&math::zero()) {
            mdir = mdir.normalize();
        }
        self.cam.pos_add(&(mdir * self.move_speed * (delta_time as Coord) ));
    }
}

impl CameraController for FPSCameraController {
    fn on_frame<'b, 'c, 'a, I: Iterator<Item = &'a Event>>(&mut self,
                                                       event_iter: I,
                                                       set_cursor_pos: &'b mut FnMut(i32, i32),
                                                       cur_locker: &'c mut FnMut(bool)) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tp);
        let delta_time = dt.as_secs() as Coord + 0.1e-8 * (dt.subsec_nanos() as Coord);
        self.last_tp = now;

        for event in event_iter {
            match *event {
                Event::KeyboardInput(el_state, _, Some(key)) => {
                    let pressed = el_state == ElementState::Pressed;
                    match key {
                        VirtualKeyCode::W | VirtualKeyCode::Up => self.move_forward = pressed,
                        VirtualKeyCode::A | VirtualKeyCode::Left => self.move_left = pressed,
                        VirtualKeyCode::D | VirtualKeyCode::Right => self.move_right = pressed,
                        VirtualKeyCode::S | VirtualKeyCode::Down => self.move_backward = pressed,
                        VirtualKeyCode::Space => self.move_up = pressed,
                        VirtualKeyCode::LShift | VirtualKeyCode::RShift => self.move_down = pressed,
                        _ => (),
                    }
                }
                Event::MouseMoved(x, y) => {
                    if self.cursor_locked {
                        let cx = self.cam.width() / 2;
                        let cy = self.cam.height() / 2;
                        let dx = cx as i32 - x;
                        let dy = cy as i32 - y;
                        let rx = dx as Coord / self.cam.width() as Coord * self.mouse_sens * self.cam.fovx();
                        let ry = dy as Coord / self.cam.height() as Coord * self.mouse_sens * self.cam.fovx();

                        self.cam.yaw_add(ry).pitch_add(rx);
                        set_cursor_pos(cx as i32, cy as i32);
                    }
                }
                Event::MouseInput(pressed, MouseButton::Right) => {
                    self.cursor_locked = pressed == ElementState::Pressed; 
                    cur_locker(self.cursor_locked);            
                    set_cursor_pos((self.cam.width() / 2) as i32,
                                   (self.cam.height() / 2) as i32);                    
                }

                _ => {}
            }
        }
        self.apply_move(delta_time);
    }
}
