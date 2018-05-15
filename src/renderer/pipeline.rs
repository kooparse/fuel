use gl;

use renderer::RendererConfig;
use renderer::Shader;
use renderer::Texture;

#[derive(Clone, Debug)]
pub struct Pipeline<'a> {
    pub config: RendererConfig<'a>,
    pub texture: Texture,
    pub shader: Shader,
    show_texture: bool,
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
            show_texture: true,
        }
    }

    pub fn set_texture_off(&mut self) {
        self.show_texture = false
    }

    pub fn set_texture_true(&mut self) {
        self.show_texture = true
    }

    // Draw our stuff
    pub fn render(&mut self) {
        let mut texture_id: u32 = 0;

        if self.show_texture {
            texture_id = self.config.texture_id;
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::BindVertexArray(self.config.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
