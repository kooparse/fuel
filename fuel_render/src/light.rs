use fuel_camera::{Projection, View};
use fuel_core::{ObjectTypes, SceneObject};
use fuel_types::{Rotation, Transform, VAO, VBO};
use fuel_utils::primitive;
use gl;
use gl::types::*;
use na::{Isometry3, Vector3};
use shader::Shader;
use std::mem;
use vertex::Vertex;

pub struct Light {
    transform: Transform,
    shader: Shader,
    vertices: Vec<f32>,
    vbo: VBO,
    vao: VAO,
}

impl Default for Light {
    fn default() -> Light {
        Self::new()
    }
}

impl Light {
    pub fn new() -> Self {
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        let shader = Shader::new("light");

        let vertices = primitive::get_cube_vertices();
        let transform = Transform {
            rotation: Rotation::new_with_default(0., -5., 0.),
            ..Default::default()
        };

        let mut light = Light {
            transform,
            shader,
            vertices,
            vao: 0,
            vbo: 0,
        };

        let (vbo, vao) = light.set_vertex(stride);
        light.vbo = vbo;
        light.vao = vao;
        light
    }
}

impl Vertex for Light {
    fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }
}

impl SceneObject for Light {
    fn set_scale(&mut self, scale: f32) {
        self.transform.scale.set(scale);
    }

    fn set_color(&self, name: &str, color: Vector3<f32>) {
        self.shader.use_program();
        self.shader.set_color(name, color);
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position.set(x, y, z);
    }

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::LIGHT
    }

    fn render(&self, projection: Projection, view: View) {
        let (position, rotation, scale) = self.transform.get();
        let mut model = Isometry3::new(position, rotation).to_homogeneous();
        model = model.append_scaling(scale);

        self.shader.use_program();
        self.shader.set_mvp(projection * view * model);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
