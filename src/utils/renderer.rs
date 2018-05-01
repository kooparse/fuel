use gl;
use gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use utils::shader::Shader;

pub unsafe fn create_shader_program(
    vertices: &[f32],
    indices: &[i32],
) -> (Shader, u32) {
    // Build and compile vertex shader
    let shader = Shader::new("src/shaders/shader.vs", "src/shaders/shader.fs");

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    // Generate vbo/ebo buffers with id
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);
    // Bind array vertex data to vbo
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        // Tell the GPU if our data are likely to change frequently
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &indices[0] as *const i32 as *const c_void,
        gl::STATIC_DRAW,
    );
    // Configure OpenGL to understand our vao
    gl::VertexAttribPointer(
        0,
        // Array is divided into variables of 3 floats each
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    // Draw wireframe polygons if flag is set
    #[cfg(feature = "wireframe")]
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    return (shader, vao);
}
