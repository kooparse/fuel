#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra as na;

mod camera;
mod renderer;
mod utils;

use glutin::Event::WindowEvent;
use glutin::WindowEvent::{CloseRequested, CursorMoved, KeyboardInput,
                          MouseInput, Resized};
use glutin::{ElementState, MouseButton, MouseCursor, VirtualKeyCode};
use glutin::{EventsLoop, GlContext, GlWindow};
use na::{Isometry3, Vector3};
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use camera::CameraMovement;
use renderer::Pipeline;

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGT: f32 = 600.;

fn main() -> Result<(), Box<Error>> {
    let mut window_loop = EventsLoop::new();
    let win_conf = glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions(WINDOW_WIDTH as u32, WINDOW_HEIGT as u32);

    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = GlWindow::new(win_conf, context, &window_loop)?;

    let window = gl_window.window();

    let mut cam =
        camera::FirstPerson::new((WINDOW_WIDTH, WINDOW_HEIGT), 45., 0.1, 100.);
    let projection = cam.get_projection();

    window
        .set_cursor_position(cam.last_pos.0 as i32, cam.last_pos.1 as i32)
        .expect("Failed to set cursor position at the center of the screen");
    // Hide cursor
    window.set_cursor(MouseCursor::NoneCursor);

    // Set current context
    unsafe { gl_window.make_current()? }

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
        Vector3::new(0., 0., 0.),
        Vector3::new(2., 0., 3.),
        Vector3::new(-2., 0., 2.),
        Vector3::new(1., 3., 6.),
        Vector3::new(-5., -3., 10.),
    ];

    let mut pipeline =
        Pipeline::new(&vertices, "cube.vs", "cube.fs", "container.jpg");
    pipeline.shader.use_program();

    let now = Instant::now();
    let mut is_mouse_right_pressed = false;
    let mut running = true;
    let mut dt: f32;
    let mut last_frame: f32 = 0.;

    while running {
        let current_frame = utils::duration_to_secs(now.elapsed()) as f32;
        dt = current_frame - last_frame;
        last_frame = current_frame;
        // set delta time for each frame
        cam.set_dt(dt);

        window_loop.poll_events(|e| {
            if let WindowEvent { event, .. } = e {
                match event {
                    MouseInput {
                        button: MouseButton::Right,
                        state,
                        ..
                    } => {
                        is_mouse_right_pressed = state == ElementState::Pressed
                    }
                    CursorMoved {
                        position: (pos_x, pos_y),
                        ..
                    } => if is_mouse_right_pressed {
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
                        Some(VirtualKeyCode::F) => {
                            pipeline.config.set_fill_mode()
                        }
                        Some(VirtualKeyCode::P) => {
                            pipeline.config.set_line_mode()
                        }
                        Some(VirtualKeyCode::L) => {
                            pipeline.config.set_point_mode()
                        }
                        Some(VirtualKeyCode::Escape) => running = false,
                        _ => (),
                    },
                    CloseRequested => running = false,
                    Resized(w, h) => gl_window.resize(w, h),
                    _ => (),
                }
            }
        });

        let view = cam.get_view();

        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for (i, position) in cube_positions.iter().enumerate() {
                let angle = 20. * i as f32;
                let rotation = Vector3::new(
                    current_frame,
                    -current_frame,
                    angle - current_frame,
                );

                let model =
                    Isometry3::new(*position, rotation).to_homogeneous();

                // Order is very important
                let mvp = projection * view * model;
                pipeline.shader.set_mvp(mvp);
                pipeline.render();
            }
        }

        gl_window.swap_buffers()?;
        sleep(Duration::from_millis(16));
    }

    Ok(())
}
