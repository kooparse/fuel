#[derive(Debug, Clone)]
pub struct Scale {
    scale: f32,
}

impl Scale {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_default(scale: f32) -> Self {
        Scale { scale }
    }

    pub fn get(&self) -> f32 {
        self.scale
    }

    pub fn set(&mut self, scale: f32) {
        self.scale = scale;
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale { scale: 1. }
    }
}
