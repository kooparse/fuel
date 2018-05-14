#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate fuel;

use fuel::FirstPerson;
use fuel::Pipeline;
use fuel::Window;
use fuel::gl;
use fuel::na::{Isometry3, Vector3};
use fuel::utils;

use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    let now = Instant::now();
    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut dt: f32;
    let mut last_frame: f32 = 0.;
    let mut control = utils::Control::new();

    win.make_current();
    win.load_gl_methods();

    let mut cam =
        FirstPerson::new((WINDOW_WIDTH, WINDOW_HEIGHT), 45., 0.1, 100.);
    let projection = cam.get_projection();

    win.gl_window
        .window()
        .set_cursor_position(cam.last_pos.0 as i32, cam.last_pos.1 as i32)
        .expect("Failed to set cursor position at the center of the screen");

    let cube_vertices = utils::get_cube_vertices();
    let cube_positions: [Vector3<f32>; 5] = [
        Vector3::new(0., 0., 0.),
        Vector3::new(2., 0., 3.),
        Vector3::new(-2., 0., 2.),
        Vector3::new(1., 3., 6.),
        Vector3::new(-5., -3., 10.),
    ];

    let mut pipeline = Pipeline::new(
        &cube_vertices,
        "cube.vs",
        "cube.fs",
        "container.jpg",
    );
    pipeline.shader.use_program();

    while control.is_running {
        let current_frame = utils::duration_to_secs(now.elapsed()) as f32;
        dt = current_frame - last_frame;
        last_frame = current_frame;
        // set delta time for each frame
        cam.set_dt(dt);

        win.event_loop.poll_events(|e| {
            control.process_inputs(e, &mut cam, &mut pipeline);
        });

        let view = cam.get_view();

        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (i, position) in cube_positions.iter().enumerate() {
            let angle = 20. * i as f32;
            let rotation = Vector3::new(
                current_frame,
                -current_frame,
                angle - current_frame,
            );

            let model = Isometry3::new(*position, rotation).to_homogeneous();

            // Order is very important
            let mvp = projection * view * model;
            pipeline.shader.set_mvp(mvp);
            pipeline.render();
        }

        win.clear_gl();
        sleep(Duration::from_millis(16));
    }
}
