use na::Vector3;
use renderer::Component;

pub type VAO = u32;
pub type VBO = u32;
pub type Position = Vector3<f32>;
pub type Rotation = Vector3<f32>;
pub type ComponentList = Vec<Box<Component>>;
