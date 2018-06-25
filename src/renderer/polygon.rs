use camera::{Projection, View};
use fuel_types::{Rotation, Transform, VAO, VBO};
use gl;
use gl::types::*;
use na::{Isometry3, Vector3};
use renderer::shader::Shader;
use renderer::texture::Texture;
use renderer::vertex::Vertex;
use scene::{ObjectTypes, SceneObject};
use std::mem;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub transform: Transform,
    pub shader: Shader,
    pub texture: Option<Texture>,
    vertices: Vec<f32>,
    vao: VAO,
    vbo: VBO,
}

impl Polygon {
    pub fn new(
        vertices: Vec<f32>,
        shader_name: &str,
        texture_name: Option<&str>,
    ) -> Polygon {
        // TODO: Find a way to auto set stride
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        let shader = Shader::new(shader_name);
        let transform = Transform {
            rotation: Rotation::new_with_default(0., 5., 0.),
            ..Default::default()
        };

        let mut object = Polygon {
            transform,
            shader,
            texture: None,
            vertices,
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

impl Vertex for Polygon {
    fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }
}

impl SceneObject for Polygon {
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
        ObjectTypes::POLYGON
    }

    fn render(&self, projection: Projection, view: View) {
        let (position, rotation, scale) = self.transform.get();
        let mut model = Isometry3::new(position, rotation).to_homogeneous();
        model = model.append_scaling(scale);

        self.shader.use_program();
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
