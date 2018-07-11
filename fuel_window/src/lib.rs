extern crate gl;
extern crate glutin;

mod control;

pub use control::Control;
use glutin::Event::WindowEvent;
use glutin::WindowEvent::{
    CloseRequested, CursorMoved, KeyboardInput, MouseInput, Resized,
};
use glutin::{
    ElementState, KeyboardInput as KInputEvents, MouseButton, MouseCursor,
    VirtualKeyCode,
};
use glutin::{EventsLoop, GlContext, GlWindow};
use std::time::{Duration, Instant};

fn duration_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + f64::from(dur.subsec_nanos()) / 1_000_000_000.0
}

pub struct Window {
    // OpenGL context and a Window with which it is associated
    pub gl_window: GlWindow,
    pub event_loop: EventsLoop,
    #[allow(dead_code)]
    dimensions: (f32, f32),

    started_time: Instant,
    delta_time: f32,
    last_frame_time: f32,
}

impl Window {
    pub fn new(title: &str, width: f32, height: f32) -> Window {
        let window_loop = EventsLoop::new();
        let win_conf = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width as u32, height as u32);

        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = GlWindow::new(win_conf, context, &window_loop).unwrap();

        // Hide cursor
        gl_window.set_cursor(MouseCursor::NoneCursor);

        Window {
            started_time: Instant::now(),
            gl_window,
            event_loop: window_loop,
            dimensions: (width, height),
            delta_time: 0.,
            last_frame_time: 0.,
        }
    }

    pub fn get_dt(&self) -> f32 {
        self.delta_time
    }

    pub fn set_cursor_position(&self, position: (f32, f32)) {
        let (last_pos_x, last_pos_y) = position;
        self.gl_window
            .window()
            .set_cursor_position(last_pos_x as i32, last_pos_y as i32)
            .expect(
                "Failed to set cursor position at the center of the screen",
            );
    }

    pub fn compute_delta(&mut self) {
        let current_frame =
            duration_to_secs(self.started_time.elapsed()) as f32;
        self.delta_time = current_frame - self.last_frame_time;
        self.last_frame_time = current_frame;
    }

    // Set current context for winit window
    pub fn make_current(&self) {
        unsafe { self.gl_window.make_current().unwrap() }
    }

    #[allow(dead_code)]
    pub fn get_dimensions(&self) -> (f32, f32) {
        self.dimensions
    }

    pub fn clear_gl(&self) {
        self.gl_window.swap_buffers().unwrap();
    }

    #[allow(dead_code)]
    pub fn resize(&self, width: u32, height: u32) {
        self.gl_window.resize(width, height);
    }

    pub fn load_gl_methods(&self) {
        gl::load_with(|symbol| {
            self.gl_window.get_proc_address(symbol) as *const _
        })
    }

    pub fn on_resize(&self, control: &Control, mut cb: impl FnMut((u32, u32))) {
        if let Some(size) = control.window_resized {
            cb(size);
        }
    }

    pub fn on_cursor_position(
        &self,
        control: &Control,
        mut cb: impl FnMut((f32, f32)),
    ) {
        cb(control.cursor_position)
    }

    pub fn on_pressed_key(
        &self,
        control: &Control,
        keycode: VirtualKeyCode,
        mut cb: impl FnMut(),
    ) {
        if control.keycode_pressed.contains_key(&Some(keycode)) {
            cb();
        }
    }

    pub fn pull_events(&mut self, control: &mut Control) {
        self.event_loop.poll_events(|glutin_events| {
            if let WindowEvent { event, .. } = glutin_events {
                match event {
                    MouseInput {
                        button: MouseButton::Right,
                        state,
                        ..
                    } => {
                        control.is_mouse_right_pressed =
                            state == ElementState::Pressed
                    }
                    CursorMoved {
                        position: (pos_x, pos_y),
                        ..
                    } => if control.is_mouse_right_pressed {
                        control.cursor_position = (pos_x as f32, pos_y as f32)
                    },
                    KeyboardInput {
                        input:
                            KInputEvents {
                                state,
                                virtual_keycode,
                                ..
                            },
                        ..
                    } => match state {
                        ElementState::Pressed => {
                            control
                                .keycode_pressed
                                .insert(virtual_keycode, state);
                        }
                        ElementState::Released => {
                            control.keycode_pressed.remove(&virtual_keycode);
                        }
                    },
                    CloseRequested => control.stop(),
                    Resized(w, h) => control.window_resized = Some((w, h)),
                    _ => (),
                }
            }
        });
    }
}
