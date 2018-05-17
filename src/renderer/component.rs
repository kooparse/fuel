use na::Matrix4;

pub enum ComponentTypes {
    OBJECT,
    LIGHT,
    CAMERA,
}


pub trait Component {
    fn set_position(&mut self, f32, f32, f32);
    fn get_type(&self) -> ComponentTypes;
    fn render(&self, Matrix4<f32>, Matrix4<f32>);
}

