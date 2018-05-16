use gl;
use na::Matrix4;
use renderer::{Component, Object};
// use renderer::ComponentList;




pub struct Scene<'a> {
    objects: Vec<Box<&'a Component>>,
}

impl <'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            objects: vec![],
        }
    }

    // Draw wireframe polygons
    pub fn set_line_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    pub fn set_point_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT) }
    }

    pub fn set_fill_mode(&self) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) }
    }

    pub fn add(&mut self, element: &'a Object) {
        self.objects.push(Box::new(element));
    }

    // Draw all component into the created scene
    pub fn render(&self, projection: Matrix4<f32>, view: Matrix4<f32>) {
        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for comp in &self.objects {
            comp.render(projection, view);
        }
    }
}
