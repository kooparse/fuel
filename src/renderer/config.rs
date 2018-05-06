use gl;
use gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use renderer::Texture;

pub struct RendererConfig<'a> {
    pub vertices: &'a [f32],
    pub vbo: u32,
    pub vao: u32,
    pub texture_id: u32,
    pub stride: GLsizei,
}

impl<'a> RendererConfig<'a> {
    pub fn new(vertices: &'a [f32], texture: Texture) -> RendererConfig<'a> {
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        let mut config = RendererConfig {
            vao: 0,
            vbo: 0,
            vertices,
            stride,
            texture_id: texture.id,
        };

        unsafe {
            // Order is important
            config.set_vertice(vertices);
            config.set_texture(texture);
        }

        return config;
    }

    pub fn set_line_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    pub fn set_point_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT) }
    }

    pub fn set_fill_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) }
    }

    unsafe fn set_vertice(&mut self, vertices: &[f32]) {
        let (mut vao, mut vbo) = (0, 0);

        gl::Enable(gl::DEPTH_TEST);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        // Generate vbo/ebo buffers with id
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            // Tell the GPU if our data are likely to change frequently
            gl::STATIC_DRAW,
        );

        // For aPos data
        gl::VertexAttribPointer(
            0,
            // Array is divided into variables of 3 floats each
            3,
            gl::FLOAT,
            gl::FALSE,
            self.stride,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Draw wireframe polygons if flag is set
        #[cfg(feature = "wireframe")]
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        self.vbo = vbo;
        self.vao = vao;
    }

    unsafe fn set_texture(&mut self, texture: Texture) {
        let mut texture_id = texture.id;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            self.stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as i32,
        );

        // Texture filter paramenters
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_LINEAR as i32,
        );

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            texture.width,
            texture.height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &texture.raw_pixels[0] as *const u8 as *const c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
        self.texture_id = texture_id;
    }
}
