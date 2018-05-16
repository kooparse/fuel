#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]


pub extern crate gl;
pub extern crate glutin;
pub extern crate image;
pub extern crate nalgebra as na;
extern crate uuid;

pub use scene::Scene;
pub use window::Window;
pub use camera::FirstPerson;
pub use renderer::Light;
pub use renderer::Object;
// pub use renderer::types;

pub mod utils;
mod scene;
mod camera;
mod renderer;
mod window;


