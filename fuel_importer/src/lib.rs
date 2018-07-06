extern crate fuel_render;
extern crate fuel_types;
extern crate gltf;
extern crate nalgebra as na;

mod gltf_format;
use fuel_render::Model;

pub use gltf_format::GltfFormater;

/// Used to convert formats to Model.
pub trait Formater {
    fn to_model(self) -> Model;
}

/// Import 3D files by formatting the data
/// to fit the engine. It will create a new Model.
pub struct Importer;
impl Importer {
    pub fn from_gltf(ressource_path: &str) -> Model {
        GltfFormater::new(ressource_path).to_model()
    }
}
