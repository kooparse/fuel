use na::Vector3;
use position::Position;
use rotation::Rotation;
use scale::Scale;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Position,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Transform {
    pub fn get(&self) -> (Vector3<f32>, Vector3<f32>, f32) {
        (self.position.get(), self.rotation.get(), self.scale.get())
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Position::new(),
            rotation: Rotation::new(),
            scale: Scale::new(),
        }
    }
}
