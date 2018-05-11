#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra as na;

mod renderer;
mod utils;

use glutin::{ElementState, MouseCursor, VirtualKeyCode};
use std::str;
use std::thread::sleep;
use std::time::{Duration, Instant};
use glutin::{EventsLoop, GlContext, GlWindow};
use na::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use glutin::Event::WindowEvent;
use glutin::WindowEvent::{CloseRequested, CursorMoved, KeyboardInput, Resized};

use renderer::Pipeline;

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGT: f32 = 600.;

fn main() {
    let mut window_loop = EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions(WINDOW_WIDTH as u32, WINDOW_HEIGT as u32);

    // gl_window..set_mouse_position(last_cursor_pos_x, last_cursor_pos_y);
    let context = glutin::ContextBuilder::new().with_vsync(true);

    let gl_window = GlWindow::new(window, context, &window_loop).unwrap();

    //
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
        Vector3::new(0., 0., 0.),
        Vector3::new(2., 0., 3.),
        Vector3::new(-2., 0., 2.),
        Vector3::new(1., 3., 6.),
        Vector3::new(-5., -3., 10.),
    ];

    let screen_ratio = WINDOW_WIDTH / WINDOW_HEIGT;
    let mut pipeline =
        Pipeline::new(&vertices, "cube.vs", "cube.fs", "container.jpg");
    pipeline.shader.use_program();

    // MVP with main camera
    let projection =
        Perspective3::new(screen_ratio, 45., 0.1, 100.).to_homogeneous();

    let mut rotate_direction: i8 = -1;

    let start = Instant::now();
    let mut running = true;

    // Intialize cursor position to be at
    // the center of the screen
    let mut last_cursor_pos_x: f32 = WINDOW_WIDTH / 2.;
    let mut last_cursor_pos_y: f32 = WINDOW_HEIGT / 2.;
    let mut first_cursor_move: bool = true;
    let mut yaw: f32 = -90.;
    let mut pitch: f32 = 0.;

    let mut camera_pos = Vector3::new(0., 0., 3.);
    let mut camera_front = Vector3::new(0., 0., -1.);
    let camera_up = Vector3::new(0., 1., 0.);

    let mut dt: f32;
    let mut last_frame: f32 = 0.;

    gl_window
        .window()
        .set_cursor_position(last_cursor_pos_x as i32, last_cursor_pos_y as i32)
        .expect("Failed to set cursor position at the center");

    // Hide cursor
    gl_window.window().set_cursor(MouseCursor::NoneCursor);

    while running {
        let current_frame = utils::duration_to_secs(start.elapsed()) as f32;
        dt = current_frame - last_frame;
        last_frame = current_frame;

        let camera_speed = 2.5 * dt;

        window_loop.poll_events(|e| {
            if let WindowEvent { event, .. } = e {
                match event {
                    CursorMoved {
                        position: (pos_x, pos_y),
                        // modifiers,
                        ..
                    } => {
                        let (pos_x, pos_y) = (pos_x as f32, pos_y as f32);
                        if first_cursor_move {
                            last_cursor_pos_x = pos_x;
                            last_cursor_pos_y = pos_y;
                            first_cursor_move = false;
                        }

                        let mut x_offset = pos_x - last_cursor_pos_x;
                        let mut y_offset = last_cursor_pos_y - pos_y;
                        last_cursor_pos_x = pos_x;
                        last_cursor_pos_y = pos_y;

                        let sensibility = 0.05;
                        x_offset *= sensibility;
                        y_offset *= sensibility;

                        yaw += x_offset;
                        pitch += y_offset;

                        if pitch > 89. {
                            pitch = 89.
                        }
                        if pitch < -89. {
                            pitch = -89.
                        }

                        camera_front = Vector3::new(
                            yaw.to_radians().cos() * pitch.to_radians().cos(),
                            pitch.to_radians().sin(),
                            yaw.to_radians().sin() * pitch.to_radians().cos(),
                        ).normalize();
                    }
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
                            camera_pos += camera_speed * camera_front;
                        }
                        Some(VirtualKeyCode::S) => {
                            camera_pos -= camera_speed * camera_front;
                        }
                        Some(VirtualKeyCode::A) => {
                            camera_pos -=
                                camera_front.cross(&camera_up) * camera_speed;
                        }
                        Some(VirtualKeyCode::D) => {
                            camera_pos +=
                                camera_front.cross(&camera_up) * camera_speed;
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
                        Some(VirtualKeyCode::R) => {
                            rotate_direction =
                                if rotate_direction == -1 { 1 } else { -1 }
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

        let eye = Point3::from_coordinates(camera_pos);
        let target = Point3::from_coordinates(camera_pos + camera_front);
        let view = Matrix4::look_at_rh(&eye, &target, &camera_up);

        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for (i, position) in cube_positions.iter().enumerate() {
                let angle = 20. * i as f32;
                let rotation = Vector3::new(
                    current_frame * f32::from(rotate_direction),
                    -current_frame * f32::from(rotate_direction),
                    angle - current_frame * f32::from(rotate_direction),
                );

                let model =
                    Isometry3::new(*position, rotation).to_homogeneous();

                // Order is very important
                let mvp = projection * view * model;
                pipeline.shader.set_mvp(mvp);
                pipeline.render();
            }
        }

        gl_window.swap_buffers().unwrap();
        sleep(Duration::from_millis(16));
    }
}
