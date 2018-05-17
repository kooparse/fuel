use na::{Matrix4, Perspective3, Point3, Vector3};
use std::default::Default;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct FirstPerson {
    pub speed: f32,
    pub win_dimensions: (f32, f32),
    fovy: f32,
    znear: f32,
    zfar: f32,

    delta_time: f32,
    sensibility: f32,
    position: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    pub last_pos: (f32, f32),
    is_first_move: bool,
}

impl FirstPerson {
    pub fn new(
        win_dimensions: (f32, f32),
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> FirstPerson {
        FirstPerson {
            win_dimensions,
            last_pos: (win_dimensions.0 / 2., win_dimensions.1 / 2.),
            fovy,
            znear,
            zfar,
            ..Default::default()
        }
    }

    pub fn get_aspect(&self) -> f32 {
        let (width, height) = self.win_dimensions;
        width / height
    }

    // Rotation of the camera
    pub fn spin_direction(&mut self, pos_x: f32, pos_y: f32) {
        if self.is_first_move {
            self.last_pos = (pos_x, pos_y);
            self.is_first_move = false;
        }

        let (last_pos_x, last_pos_y) = self.last_pos;
        let mut x_offset = pos_x - last_pos_x;
        let mut y_offset = last_pos_y - pos_y;
        self.last_pos = (pos_x, pos_y);

        x_offset *= self.sensibility;
        y_offset *= self.sensibility;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89. {
            self.pitch = 89.
        }
        if self.pitch < -89. {
            self.pitch = -89.
        }

        self.front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize();
    }
    // Moving camera on the coords in 3d space
    pub fn move_direction(&mut self, direction: &CameraMovement) {
        let speed = self.get_dt_speed();
        match direction {
            CameraMovement::FORWARD => self.position += speed * self.front,
            CameraMovement::BACKWARD => self.position -= speed * self.front,
            CameraMovement::LEFT => {
                self.position -= self.front.cross(&self.up) * speed
            }
            CameraMovement::RIGHT => {
                self.position += self.front.cross(&self.up) * speed
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.delta_time = dt
    }

    #[allow(dead_code)]
    pub fn set_fovy(&mut self, fovy: f32) {
        self.fovy = fovy;
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let eye = Point3::from_coordinates(self.position);
        let target = Point3::from_coordinates(self.position + self.front);

        Matrix4::look_at_rh(&eye, &target, &self.up)
    }

    pub fn get_projection(&self) -> Matrix4<f32> {
        Perspective3::new(
            self.get_aspect(),
            self.fovy,
            self.znear,
            self.zfar,
        ).to_homogeneous()
    }

    pub fn get_dt_speed(&self) -> f32 {
        self.speed * self.delta_time
    }
}

// Default values
impl Default for FirstPerson {
    fn default() -> FirstPerson {
        let win_dimensions = (800., 600.);
        FirstPerson {
            delta_time: 0.,
            speed: 2.5,
            win_dimensions,
            fovy: 45.,
            znear: 0.1,
            zfar: 100.,

            sensibility: 0.05,
            position: Vector3::new(0., 0., 3.),
            front: Vector3::new(0., 0., -1.),
            up: Vector3::new(0., 1., 0.),
            yaw: -90.,
            pitch: 0.,
            // Intialize cursor position to be at
            // the center of the screen
            last_pos: (win_dimensions.0 / 2., win_dimensions.1 / 2.),
            is_first_move: true,
        }
    }
}
