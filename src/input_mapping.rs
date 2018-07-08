use fuel::fuel_camera::CameraMovement;

use fuel::glutin::VirtualKeyCode;
use fuel::{Control, Scene, Window};

pub fn process_input(
    win: &mut Window,
    scene: &mut Scene,
    control: &mut Control,
) {
    win.on_resize(control, |(w, h)| win.resize(w, h));

    win.on_cursor_position(control, |(x, y)| scene.camera.spin_direction(x, y));

    win.on_pressed_key(control, VirtualKeyCode::F, || scene.set_fill_mode());
    win.on_pressed_key(control, VirtualKeyCode::L, || scene.set_line_mode());
    win.on_pressed_key(control, VirtualKeyCode::P, || scene.set_point_mode());

    win.on_pressed_key(control, VirtualKeyCode::W, || {
        scene.camera.move_direction(&CameraMovement::FORWARD)
    });

    win.on_pressed_key(control, VirtualKeyCode::S, || {
        scene.camera.move_direction(&CameraMovement::BACKWARD)
    });

    win.on_pressed_key(control, VirtualKeyCode::A, || {
        scene.camera.move_direction(&CameraMovement::LEFT)
    });

    win.on_pressed_key(control, VirtualKeyCode::D, || {
        scene.camera.move_direction(&CameraMovement::RIGHT)
    });
}
