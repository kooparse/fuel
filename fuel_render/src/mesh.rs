use fuel_camera::{Projection, View};
use fuel_core::{ObjectTypes, SceneObject};
use fuel_types::Transform;
use na::Vector3;
use primitive::Primitive;

pub type Meshes = Vec<Mesh>;

/// Model contains a list of Mesh that contains
/// a list of Primitive that contains a list of Vertex.
///
/// When we create a new Primitive, we setup automatically
/// openGL with his vertices. We don't keep it in memory.
///
/// Transform contains the position, scale and the rotation
/// of a Model.  When we update a Model, we also update the shader
/// inside the primitives.
pub struct Model {
    transform: Transform,
    meshes: Meshes,
}

impl SceneObject for Model {
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position.set(x, y, z);
    }

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::MODEL
    }

    fn render(&self, proj: Projection, view: View) {
        self.meshes.iter().for_each(|mesh| {
            mesh.draw(proj, view, &self.transform);
        });
    }

    fn set_color(&self, _name: &str, _color: Vector3<f32>) {}
    fn set_scale(&mut self, _scale: f32) {}
}

impl Model {
    pub fn new(transform: Transform, meshes: Meshes) -> Self {
        Self { transform, meshes }
    }
}

pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }
    fn draw(&self, proj: Projection, view: View, transform: &Transform) {
        self.primitives.iter().for_each(|primitive| {
            primitive.shader_config(proj, view, transform);
        })
    }
}
