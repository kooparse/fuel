use gl;
use gl::types::*;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout(location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn check_shader_compile_error(shader: GLuint) {
    unsafe {
        const CAPACITY: usize = 512;
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(CAPACITY);
        info_log.set_len(CAPACITY - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }
    }
}

pub fn create_shader_program(
    vertices: &[f32],
    indices: &[i32],
) -> (GLuint, u32) {
    unsafe {
        // Build and compile vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        check_shader_compile_error(vertex_shader);

        // Build and compile fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let str_vert = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &str_vert.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        check_shader_compile_error(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);


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

        return (shader_program, vao);
    };
}
