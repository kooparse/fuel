use glutin;
use glutin::Event::WindowEvent;
use glutin::WindowEvent::{CloseRequested, CursorMoved, KeyboardInput,
                          MouseInput};
use glutin::{ElementState, MouseButton, VirtualKeyCode};

use camera::CameraMovement;
use camera::FirstPerson;
use renderer::Pipeline;

pub struct Control {
    pub is_running: bool,
    is_mouse_right_pressed: bool,
}

impl Control {
    pub fn new() -> Control {
        Control {
            is_running: true,
            is_mouse_right_pressed: false,
        }
    }

    pub fn process_inputs(
        &mut self,
        e: glutin::Event,
        cam: &mut FirstPerson,
        pipeline: &mut Pipeline,
    ) {
        if let WindowEvent { event, .. } = e {
            match event {
                MouseInput {
                    button: MouseButton::Right,
                    state,
                    ..
                } => {
                    self.is_mouse_right_pressed = state == ElementState::Pressed
                }
                CursorMoved {
                    position: (pos_x, pos_y),
                    ..
                } => if self.is_mouse_right_pressed {
                    cam.spin_direction(pos_x as f32, pos_y as f32);
                },
                KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => match virtual_keycode {
                    Some(VirtualKeyCode::W) => {
                        cam.move_direction(&CameraMovement::FORWARD)
                    }
                    Some(VirtualKeyCode::S) => {
                        cam.move_direction(&CameraMovement::BACKWARD)
                    }
                    Some(VirtualKeyCode::A) => {
                        cam.move_direction(&CameraMovement::LEFT)
                    }
                    Some(VirtualKeyCode::D) => {
                        cam.move_direction(&CameraMovement::RIGHT)
                    }
                    Some(VirtualKeyCode::F) => pipeline.config.set_fill_mode(),
                    Some(VirtualKeyCode::P) => pipeline.config.set_line_mode(),
                    Some(VirtualKeyCode::L) => pipeline.config.set_point_mode(),
                    Some(VirtualKeyCode::Escape) => self.is_running = false,
                    _ => (),
                },
                CloseRequested => self.is_running = false,
                // TODO: Fight and win the borrow checker
                // Resized(w, h) => window.resize(w, h),
                _ => (),
            }
        }
    }
}
