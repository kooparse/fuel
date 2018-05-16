extern crate fuel;

use fuel::Window;
use fuel::utils;
use fuel::{FirstPerson, Object, Scene};
use std::thread::sleep;
use std::time::Duration;

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
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

    let mut scene = Scene::new();
    let mut cube = Object::new(
        utils::get_cube_vertices(),
        "cube",
        Some("container.jpg"),
    );
    cube.set_position(0., 0., -3.);
    scene.add(&cube);
    // cube.set_position(0., 0., 0.);

    while control.is_running {
        win.clear_gl();
        win.event_loop.poll_events(|e| {
            control.process_inputs(e, &mut cam, &mut scene);
        });
        let view = cam.get_view();
        // Render all components into the
        // current scene
        scene.render(projection, view);
        sleep(Duration::from_millis(16));
    }
}
