use fuel_utils::time;
use gl;
use glutin;
use glutin::Event::WindowEvent;
use glutin::WindowEvent::KeyboardInput;
use glutin::{ElementState, MouseCursor, VirtualKeyCode};
use glutin::{EventsLoop, GlContext, GlWindow};
use std::time::Instant;

pub struct Window {
    // OpenGL context and a Window with which it is associated
    pub gl_window: GlWindow,
    pub event_loop: EventsLoop,
    #[allow(dead_code)]
    is_running: bool,
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
            is_running: true,
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
            time::duration_to_secs(self.started_time.elapsed()) as f32;
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

    // TODO: Be able to call pressed key like that
    // win.on_pressed_key(VirtualKeyCode::W, || {
    //     cam.move_direction(&CameraMovement::FORWARD)
    // });
    #[allow(dead_code)]
    pub fn on_pressed_key(
        &mut self,
        keycode: VirtualKeyCode,
        mut f: impl FnMut(),
    ) {
        self.event_loop.poll_events(|e| {
            if let WindowEvent { event, .. } = e {
                if let KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                } = event
                {
                    // TODO: Remove unwrap
                    if virtual_keycode.unwrap() == keycode {
                        f()
                    }
                }
            }
        })
    }
}
