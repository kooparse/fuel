use camera::FirstPerson;
use gl;
use renderer::component::Component;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct Scene {
    objects: HashMap<Uuid, Box<Component>>,
    pub camera: FirstPerson,
}

impl Scene {
    pub fn new(width: f32, height: f32, fov: f32, near: f32, far: f32) -> Self {
        let camera = FirstPerson::new((width, height), fov, near, far);
        Scene {
            camera,
            ..Default::default()
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

    pub fn add(&mut self, element: impl Component + 'static) -> Uuid {
        let key = Uuid::new_v4();
        self.objects.insert(key, Box::new(element));
        key
    }

    pub fn get_component(&mut self, key: Uuid) -> &mut Component {
        self.objects
            .get_mut(&key)
            .map(|x| &mut **x)
            .expect("Failed to retrieve component")
    }

    // Draw all component into the created scene
    pub fn render(&self) {
        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let projection = self.camera.get_projection();
        let view = self.camera.get_view();

        for component in self.objects.values() {
            component.render(projection, view);
        }
    }
}
