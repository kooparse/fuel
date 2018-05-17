extern crate fuel;

use fuel::Window;
use fuel::utils::Control;
use fuel::utils::primitive;
use fuel::utils::time;
use fuel::{Object, Scene};
use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    let now: Instant = Instant::now();

    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut control = Control::new();
    let mut scene = Scene::new(WINDOW_WIDTH, WINDOW_HEIGHT, 45., 0.1, 100.);

    let mut dt: f32;
    let mut last_frame: f32 = 0.;

    win.make_current();
    win.load_gl_methods();

    let (last_pos_x, last_pos_y) = scene.camera.last_pos;
    win.gl_window
        .window()
        .set_cursor_position(last_pos_x as i32, last_pos_y as i32)
        .expect("Failed to set cursor position at the center of the screen");

    let cube = Object::new(
        primitive::get_cube_vertices(),
        "cube",
        Some("container.jpg"),
    );

    let id = scene.add(cube);
    scene.get_component(id).set_position(0., 0., 0.);

    while control.is_running {
        win.clear_gl();
        let current_frame = time::duration_to_secs(now.elapsed()) as f32;
        dt = current_frame - last_frame;
        last_frame = current_frame;
        // set delta time for each frame
        scene.camera.set_dt(dt);

        win.event_loop.poll_events(|e| {
            control.process_inputs(e, &mut scene);
        });
        // Render all components into the
        // current scene
        scene.render();
        sleep(Duration::from_millis(16));
    }
}
