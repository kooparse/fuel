extern crate fuel;

use fuel::Window;
use fuel::utils::Control;
use fuel::utils::primitive;
use fuel::utils::time;
use fuel::{FirstPerson, Object, Scene};
use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    let now: Instant = Instant::now();

    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut control = Control::new();
    let mut scene = Scene::new();

    let mut dt: f32;
    let mut last_frame: f32 = 0.;

    win.make_current();
    win.load_gl_methods();

    let mut cam =
        FirstPerson::new((WINDOW_WIDTH, WINDOW_HEIGHT), 45., 0.1, 100.);
    let projection = cam.get_projection();

    win.gl_window
        .window()
        .set_cursor_position(cam.last_pos.0 as i32, cam.last_pos.1 as i32)
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
        cam.set_dt(dt);

        let view = cam.get_view();
        win.event_loop.poll_events(|e| {
            control.process_inputs(e, &mut cam, &mut scene);
        });
        // Render all components into the
        // current scene
        scene.render(projection, view);
        sleep(Duration::from_millis(16));
    }
}
