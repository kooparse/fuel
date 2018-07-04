use fuel_camera::{FirstPerson, Projection, View};
use gl;
use na::Vector3;
use std::collections::HashMap;
use uuid::Uuid;

// Enum of all type that an object
// can have
pub enum ObjectTypes {
    POLYGON,
    LIGHT,
    MODEL,
}

// Object to put in the scene
// Position in the spacial scene with render method
// is needed
pub trait SceneObject {
    fn set_position(&mut self, f32, f32, f32);
    fn get_type(&self) -> ObjectTypes;
    fn render(&self, Projection, View);
    fn set_color(&self, name: &str, Vector3<f32>);
    fn set_scale(&mut self, scale: f32);
}

#[derive(Default)]
pub struct Scene {
    objects: HashMap<Uuid, Box<SceneObject>>,
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

    pub fn add(&mut self, element: impl SceneObject + 'static) -> Uuid {
        let key = Uuid::new_v4();
        self.objects.insert(key, Box::new(element));
        key
    }

    pub fn get_object(&mut self, key: Uuid) -> &mut SceneObject {
        self.objects
            .get_mut(&key)
            .map(|x| &mut **x)
            .expect("Failed to retrieve object")
    }

    // Draw all object into the created scene
    pub fn render(&self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let projection = self.camera.get_projection();
        let view = self.camera.get_view();

        for object in self.objects.values() {
            object.render(projection, view);
        }
    }
}
