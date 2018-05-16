use na::Matrix4;

pub enum ComponentTypes {
    OBJECT,
    LIGHT,
}

pub trait Component {
    fn get_type(&self) -> ComponentTypes;
    fn render(&self, Matrix4<f32>, Matrix4<f32>);
    fn configuration(&self);
}
