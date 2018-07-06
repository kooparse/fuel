use glutin;
use glutin::Event::WindowEvent;
use glutin::WindowEvent::{
    CloseRequested, CursorMoved, KeyboardInput, MouseInput,
};
use glutin::{ElementState, MouseButton, VirtualKeyCode};

use fuel_camera::CameraMovement;
use fuel_core::Scene;

#[derive(Default)]
pub struct Control {
    pub is_running: bool,
    is_mouse_right_pressed: bool,
}

impl Control {
    pub fn new() -> Self {
        Control {
            is_running: true,
            ..Default::default()
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    // TODO: Be able to call pressed key like that
    // win.on_pressed_key(VirtualKeyCode::W, || {
    //     cam.move_direction(&CameraMovement::FORWARD)
    // });
    pub fn process_inputs(&mut self, e: glutin::Event, scene: &mut Scene) {
        // let mut cam = scene.camera;
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
                    scene.camera.spin_direction(pos_x as f32, pos_y as f32);
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
                        scene.camera.move_direction(&CameraMovement::FORWARD)
                    }
                    Some(VirtualKeyCode::S) => {
                        scene.camera.move_direction(&CameraMovement::BACKWARD)
                    }
                    Some(VirtualKeyCode::A) => {
                        scene.camera.move_direction(&CameraMovement::LEFT)
                    }
                    Some(VirtualKeyCode::D) => {
                        scene.camera.move_direction(&CameraMovement::RIGHT)
                    }
                    Some(VirtualKeyCode::F) => scene.set_fill_mode(),
                    Some(VirtualKeyCode::P) => scene.set_line_mode(),
                    Some(VirtualKeyCode::L) => scene.set_point_mode(),
                    Some(VirtualKeyCode::Escape) => self.stop(),
                    _ => (),
                },
                CloseRequested => self.stop(),
                // TODO: Fight and win the borrow checker
                // Resized(w, h) => window.resize(w, h),
                _ => (),
            }
        }
    }
}
