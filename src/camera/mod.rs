mod first_person;
pub use self::first_person::FirstPerson;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
