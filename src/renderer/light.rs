use gl;
use gl::types::*;

use na::Matrix4;
use std::mem;
use std::ptr;

use renderer::component::{Component, ComponentTypes};
use renderer::types::Position;

pub struct Light {
    position: Position,
    vao: u32,
}

impl Default for Light {
    fn default() -> Light {
        Self::new()
    }
}

impl Light {
    pub fn new() -> Self {
        let mut light: Light = Default::default();
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
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Position::new(x, y, z);
    }

    fn get_type(&self) -> ComponentTypes {
        ComponentTypes::LIGHT
    }
    fn render(&self, _proj: Matrix4<f32>, _view: Matrix4<f32>) {}

    fn configuration(&self) {}
}
