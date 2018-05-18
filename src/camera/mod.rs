use na::Matrix4;

mod first_person;
pub use self::first_person::FirstPerson;

pub type Projection = Matrix4<f32>;
pub type View = Matrix4<f32>;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
