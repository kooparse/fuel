use gl;

use renderer::RendererConfig;
use renderer::Shader;
use renderer::Texture;

#[derive(Clone, Debug)]
pub struct Pipeline<'a> {
    pub config: RendererConfig<'a>,
    pub texture: Texture,
    pub shader: Shader,
}

impl<'a> Pipeline<'a> {
    pub fn new(
        vertices: &'a [f32],
        vertex_shader_file: &str,
        frag_shader_file: &str,
        texture_file_path: &str,
    ) -> Pipeline<'a> {
        let shader = Shader::new(vertex_shader_file, frag_shader_file);
        let mut texture = Texture::new(texture_file_path);
        let config = RendererConfig::new(vertices, &texture.clone());
        texture.id = config.texture_id;

        Pipeline {
            config,
            texture,
            shader,
        }
    }

    // Draw our stuff
    pub fn render(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::BindVertexArray(self.config.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
