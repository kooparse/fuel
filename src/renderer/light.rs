use gl;
use gl::types::*;

use na::Matrix4;
use std::mem;
use std::ptr;

use renderer::{Component, ComponentTypes};

pub struct Light {
    vao: u32,
}

impl Light {
    pub fn new() -> Light {
        let mut light = Light { vao: 0 };
        unsafe { light.set_vao() }

        light
    }

    unsafe fn set_vao(&mut self) {
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        let mut light_vao = 0;
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, light_vao);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        self.vao = light_vao;
    }
}

impl Component for Light {
    fn get_type(&self) -> ComponentTypes {
        ComponentTypes::LIGHT
    }
    fn render(&self, _proj: Matrix4<f32>, _view: Matrix4<f32>) {}

    fn configuration(&self) {}
}
