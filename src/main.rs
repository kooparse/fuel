extern crate fuel;
extern crate gltf;

use fuel::fuel_utils::primitive;
use fuel::fuel_utils::Control;
use fuel::na::{Vector2, Vector3};
use fuel::Window;
use fuel::{Light, Model, Polygon, Scene};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const TITLE: &str = "Engine";
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;


fn main() -> Result<(), Box<Error>> {
    let mut win = Window::new(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut control = Control::new();
    let mut scene = Scene::new(WINDOW_WIDTH, WINDOW_HEIGHT, 45., 0.1, 100.);


    let mesh = Model::from_gltf("src/assets/meshes/yoshi/scene.gltf");

    win.make_current();
    win.load_gl_methods();
    win.set_cursor_position(scene.camera.last_pos);

    let cube_vertices = primitive::get_cube_vertices();
    let cube = Polygon::new(cube_vertices, "cube_light", None);
    let id = scene.add(cube);
    scene.get_object(id).set_position(0., 0., 0.);
    scene
        .get_object(id)
        .set_color("objectColor", Vector3::new(1., 0.5, 0.31));
    scene
        .get_object(id)
        .set_color("lightColor", Vector3::new(1., 1., 1.));

    let lamp = Light::new();
    let id = scene.add(lamp);
    scene.get_object(id).set_position(1., 1., -2.);
    scene.get_object(id).set_scale(0.5);

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
