use gl;
use gl::types::*;

use fuel_types::{VAO, VBO};
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub trait Vertex {
    fn get_vertices(&self) -> Vec<f32>;

    fn set_vertex(&self, stride: GLsizei) -> (VBO, VAO) {
        let vertices = self.get_vertices();
        let (mut vao, mut vbo) = (0, 0);

        unsafe {
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
                stride,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

        (vbo, vao)
    }
}
