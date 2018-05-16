use gl;
use gl::types::*;
use na::{Isometry3, Matrix4};
use renderer::types::{Position, Rotation, VAO, VBO};
use renderer::{Component, ComponentTypes};
use renderer::{Shader, Texture, VertexSetup};
use std::mem;

pub struct Object {
    scene_id: f32,
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
        let scene_id = 1.;
        let shader = Shader::new(shader_name);
        // TODO: Find a way to auto set stride
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

        shader.use_program();

        let mut object = Object {
            scene_id,
            shader,
            texture: None,
            vertices: vertices,
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

        return object;
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Position::new(x, y, z);
    }
}

impl VertexSetup for Object {
    fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }
}

impl Component for Object {
    fn get_type(&self) -> ComponentTypes {
        ComponentTypes::OBJECT
    }

    fn render(&self, projection: Matrix4<f32>, view: Matrix4<f32>) {
        let model =
            Isometry3::new(self.position, self.rotation).to_homogeneous();
        self.shader.set_mvp(projection * view * model);

        let mut texture = self.texture.clone().unwrap();
        texture.render();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
    fn configuration(&self) {}
}
