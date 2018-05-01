extern crate gl;
extern crate glutin;
extern crate nalgebra as na;

mod utils;

use std::str;
use std::ptr;
use std::time::SystemTime;
use glutin::GlContext;
use na::Vector4;

use utils::create_shader_program;

const TITLE: &str = "OpenGL";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGT: u32 = 600;

fn main() {
    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGT);

    let context = glutin::ContextBuilder::new().with_vsync(true);

    let gl_window =
        glutin::GlWindow::new(window, context, &event_loop).unwrap();

    unsafe {
        // Set current context
        gl_window.make_current().unwrap();
    }

    // Load all OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0
    ];
    let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

    let (shader, vao) = unsafe { create_shader_program(&vertices, &indices) };

    let now = SystemTime::now();
    let mut running = true;

    while running {
        let ellapsed = now.elapsed().unwrap();
        let ellapsed = ellapsed.as_secs() as f32;

        event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(h, w) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw our stuff
            shader.use_program();
            shader.set_color(
                "square_color",
                Vector4::new(0., ellapsed.sin(), 0., 1.),
            );

            gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        gl_window.swap_buffers().unwrap();
    }
}
