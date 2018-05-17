use gl;
use gl::types::*;
use na::{Isometry3, Matrix4};
use renderer::component::{Component, ComponentTypes};
use renderer::shader::Shader;
use renderer::texture::Texture;
use renderer::types::{Position, Rotation, VAO, VBO};
use renderer::vertex::VertexSetup;
use std::mem;

#[derive(Debug, Clone)]
pub struct Object {
    vertices: Vec<f32>,
    pub rotation: Rotation,
    pub position: Position,
    pub shader: Shader,
    pub texture: Option<Texture>,
    vao: VAO,
    vbo: VBO,
}

impl Object {
    pub fn new(
        vertices: Vec<f32>,
        shader_name: &str,
        texture_name: Option<&str>,
    ) -> Object {
        let shader = Shader::new(shader_name);
        // TODO: Find a way to auto set stride
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        shader.use_program();

        let mut object = Object {
            shader,
            vertices,
            texture: None,
            rotation: Rotation::new(0., 5., 0.),
            position: Position::new(0., 0., 0.),
            vao: 0,
            vbo: 0,
        };

        let (vbo, vao) = object.set_vertex(stride);

        let texture = match texture_name {
            Some(x) => Some(Texture::new(x, stride)),
            None => None,
        };

        object.texture = texture;
        object.vbo = vbo;
        object.vao = vao;
        object
    }
}

impl VertexSetup for Object {
    fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }
}

impl Component for Object {
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Position::new(x, y, z);
    }

    fn get_type(&self) -> ComponentTypes {
        ComponentTypes::OBJECT
    }

    fn render(&self, projection: Matrix4<f32>, view: Matrix4<f32>) {
        let model =
            Isometry3::new(self.position, self.rotation).to_homogeneous();
        self.shader.set_mvp(projection * view * model);

        if let Some(mut texture) = self.texture.clone() {
            texture.render();
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
