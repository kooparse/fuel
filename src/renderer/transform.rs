use renderer::types::{Position, Rotation, Scale};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Position,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Transform {
    pub fn get(&self) -> (Position, Rotation) {
        (self.position, self.rotation)
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos
    }

    pub fn set_rotation(&mut self, rot: Rotation) {
        self.rotation = rot
    }

    pub fn get_scale(&self) -> Scale {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Position::new(0., 0., 0.),
            rotation: Rotation::new(0., 0., 0.),
            scale: 1.,
        }
    }
}
