use gl;
use gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::path::PathBuf;
use image;
use image::GenericImage;

use utils::shader::Shader;

pub unsafe fn create_shader_program(
    vertices: &[f32],
    indices: &[i32],
) -> (Shader, u32, u32) {
    // Build and compile vertex shader
    let shader = Shader::new(
        "src/assets/shaders/shader.vs",
        "src/assets/shaders/shader.fs",
    );

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
    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
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

    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);

    // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // gl::BindVertexArray(0);

    // Texture
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    // Texture wrapping paramenters
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

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/assets/textures/container.jpg");
    println!("{:?}", path);
    let img = image::open(path).expect("Failed to load texture");
    let img_data = img.raw_pixels();

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &img_data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    // Draw wireframe polygons if flag is set
    #[cfg(feature = "wireframe")]
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    return (shader, vao, texture);
}
