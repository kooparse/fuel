use na::Vector3;

#[derive(Default, Debug, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_default(x: f32, y: f32, z: f32) -> Self {
        Position { x, y, z }
    }

    pub fn get(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
