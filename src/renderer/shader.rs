use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;
use std::ffi::CString;
use std::path::PathBuf;

use gl;
use gl::types::*;
use na::{Matrix4, Vector4};

#[derive(Clone, Debug)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        let vertex_shader =
            shader.compile_shader(gl::VERTEX_SHADER, vertex_path);
        let fragment_shader =
            shader.compile_shader(gl::FRAGMENT_SHADER, fragment_path);

        unsafe {
            let shader_program_id = gl::CreateProgram();
            gl::AttachShader(shader_program_id, vertex_shader);
            gl::AttachShader(shader_program_id, fragment_shader);
            gl::LinkProgram(shader_program_id);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader.id = shader_program_id;
        }

        shader
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    #[allow(dead_code)]
    pub fn set_bool(&self, var_name: &str, value: bool) {
        let shader_variable = self.get_location(var_name);
        unsafe { gl::Uniform1i(shader_variable, value as i32) }
    }

    #[allow(dead_code)]
    pub fn set_int(&self, var_name: &str, value: i32) {
        let shader_variable = self.get_location(var_name);
        unsafe { gl::Uniform1i(shader_variable, value) }
    }

    #[allow(dead_code)]
    pub fn set_float(&self, var_name: &str, value: f32) {
        let shader_variable = self.get_location(var_name);
        unsafe { gl::Uniform1f(shader_variable, value) }
    }

    #[allow(dead_code)]
    pub fn set_color(&self, var_name: &str, rgba: Vector4<f32>) {
        let shader_variable = self.get_location(var_name);
        unsafe {
            gl::Uniform4f(shader_variable, rgba.x, rgba.y, rgba.z, rgba.w)
        }
    }

    pub fn set_mvp(&self, mvp: Matrix4<f32>) {
        self.set_matrix4("mvp", mvp.as_slice());
    }

    #[allow(dead_code)]
    pub fn set_transform(&self, transform: Matrix4<f32>) {
        self.set_matrix4("transform", transform.as_slice());
    }

    pub fn set_matrix4(&self, var_name: &str, transform: &[f32]) {
        let shader_variable = self.get_location(var_name);
        unsafe {
            gl::UniformMatrix4fv(
                shader_variable,
                1,
                gl::FALSE,
                transform.as_ptr(),
            );
        }
    }

    fn get_location(&self, var_name: &str) -> GLint {
        let var_name = CString::new(var_name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, var_name.as_ptr()) }
    }

    unsafe fn check_shader_compile_error(&self, shader: GLuint) {
        const CAPACITY: usize = 512;
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(CAPACITY);
        info_log.set_len(CAPACITY - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
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

    fn compile_shader(&self, shader_type: GLenum, file_path: &str) -> u32 {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/assets/shaders");
        path.push(file_path);
        let mut shader_file = File::open(path).unwrap();
        let mut shader_string = String::new();

        // Transform file to string and store it in a variable
        shader_file
            .read_to_string(&mut shader_string)
            .expect("Failed to read vertex shader file");

        // convert to C compatible string
        let shader_source_string =
            CString::new(shader_string.as_bytes()).unwrap();

        unsafe {
            let shader = gl::CreateShader(shader_type);

            gl::ShaderSource(
                shader,
                1,
                &shader_source_string.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(shader);
            self.check_shader_compile_error(shader);

            shader
        }
    }
}
