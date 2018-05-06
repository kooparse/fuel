extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra as na;

mod renderer;
mod utils;

use glutin::VirtualKeyCode;
use std::str;
use std::thread::sleep;
use std::time::{Duration, Instant};
use glutin::GlContext;
use na::{Isometry3, Perspective3, Point3, Vector3};

use renderer::Pipeline;

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGT: f32 = 600.;

fn main() {
    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions(WINDOW_WIDTH as u32, WINDOW_HEIGT as u32);

    let context = glutin::ContextBuilder::new().with_vsync(true);

    let gl_window =
        glutin::GlWindow::new(window, context, &event_loop).unwrap();

    // Set current context
    unsafe { gl_window.make_current().unwrap() }

    // Load all OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5,
        1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5,
        -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0,
        0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5,
        0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
        0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0,
        1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
        0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
        0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
        0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
    ];

    let cube_positions: [Vector3<f32>; 5] = [
        Vector3::new(0., 0., 1.),
        Vector3::new(2., 0., 3.),
        Vector3::new(-2., 0., 2.),
        Vector3::new(1., 3., 6.),
        Vector3::new(-5., -3., 10.),
    ];

    let screen_ratio = WINDOW_WIDTH / WINDOW_HEIGT;
    let mut pipeline =
        Pipeline::new(&vertices, "cube.vs", "cube.fs", "lunar_surface.png");
    pipeline.shader.use_program();

    // MVP with main camera
    let eye = Point3::new(0.0, 0.0, -2.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
    let projection = Perspective3::new(screen_ratio, 45., 0.1, 100.);
    pipeline.shader.set_projection(projection.to_homogeneous());
    pipeline.shader.set_view(view.to_homogeneous());

    let start = Instant::now();
    let mut running = true;

    while running {
        let dt = utils::duration_to_secs(Instant::now().duration_since(start))
            as f32;

        event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::KeyboardInput { input, .. } => match input
                    .virtual_keycode
                {
                    Some(VirtualKeyCode::W) => pipeline.config.set_line_mode(),
                    Some(VirtualKeyCode::F) => pipeline.config.set_fill_mode(),
                    Some(VirtualKeyCode::P) => pipeline.config.set_point_mode(),
                    _ => (),
                },
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(h, w) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for (i, position) in cube_positions.iter().enumerate() {
                let angle = 20. * i as f32;
                let model = Isometry3::new(
                    *position,
                    Vector3::new(-dt, -dt, angle - dt),
                );

                pipeline.shader.set_model(model.to_homogeneous());
                pipeline.render();
            }
        }

        gl_window.swap_buffers().unwrap();

        sleep(Duration::from_millis(16));
    }
}
