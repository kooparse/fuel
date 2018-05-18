#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub extern crate gl;
pub extern crate glutin;
pub extern crate image;
pub extern crate nalgebra as na;
extern crate uuid;

pub use camera::FirstPerson;
pub use renderer::Light;
pub use renderer::Polygon;
pub use renderer::*;
pub use scene::{Scene, ObjectTypes};
pub use window::Window;

mod camera;
mod renderer;
mod scene;
pub mod utils;
mod window;
