use gl;
use na::Matrix4;
use renderer::component::Component;
use renderer::types::Position;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct Scene {
    objects: HashMap<Uuid, Box<Component>>,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
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

    pub fn add(&mut self, element: impl Component + 'static) -> Uuid {
        let key = Uuid::new_v4();
        self.objects.insert(key, Box::new(element));
        key
    }

    pub fn position(&mut self, key: Uuid, position: Position) {
        if let Some(x) = self.objects.get_mut(&key) {
            x.set_position(position.x, position.y, position.z);
        }
    }

    pub fn get_component(&mut self, key: Uuid) -> Option<&Component> {
        self.objects.get(&key).map(|x| &**x)
    }

    // Draw all component into the created scene
    pub fn render(&self, projection: Matrix4<f32>, view: Matrix4<f32>) {
        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for component in self.objects.values() {
            component.render(projection, view);
        }
    }
}
