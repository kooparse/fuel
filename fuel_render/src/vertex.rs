use na::{Vector2, Vector3};

#[derive(Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord_0: Vector2<f32>,
    pub tex_coord_1: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zeros(),
            normal: Vector3::zeros(),
            tex_coord_0: Vector2::zeros(),
            tex_coord_1: Vector2::zeros(),
        }
    }
}

impl Vertex {
    pub fn pos_to_array(&self) -> [f32; 3] {
        let pos = self.position;
        [pos.x, pos.y, pos.z]
    }
}
