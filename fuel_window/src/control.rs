use glutin::{ElementState, VirtualKeyCode};
use std::collections::HashMap;

#[derive(Default)]
pub struct Control {
    pub is_running: bool,
    pub window_resized: Option<(u32, u32)>,
    pub is_mouse_right_pressed: bool,
    pub keycode_pressed: HashMap<Option<VirtualKeyCode>, ElementState>,
    pub cursor_position: (f32, f32),
}

impl Control {
    pub fn new() -> Self {
        Self {
            is_running: true,
            ..Default::default()
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }
}
