extern crate fuel;
extern crate gltf;

use fuel::fuel_utils::Control;
use fuel::Importer;
use fuel::Scene;
use fuel::Window;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const TITLE: &str = "Fuel";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() -> Result<(), Box<Error>> {
    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut control = Control::new();
    let mut scene = Scene::new(WINDOW_WIDTH, WINDOW_HEIGHT, 45., 0.1, 100.);

    win.make_current();
    win.load_gl_methods();
    win.set_cursor_position(scene.camera.last_pos);

    let cube_1 = Importer::from_gltf(
        "src/assets/meshes/samples/textured/BoxTextured.gltf",
    );
    let id = scene.add(cube_1);
    scene.get_object(id).set_position(0., 0., 0.);

    while control.is_running {
        win.clear_gl();
        win.compute_delta();
        // set delta time for each frame
        scene.camera.set_dt(win.get_dt());

        win.event_loop.poll_events(|e| {
            control.process_inputs(e, &mut scene);
        });
        // Render all components into the
        // current scene
        scene.render();
        sleep(Duration::from_millis(16));
    }

    Ok(())
}
