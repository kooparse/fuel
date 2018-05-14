#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub extern crate gl;
pub extern crate glutin;
pub extern crate image;
pub extern crate nalgebra as na;

pub use renderer::Pipeline;
pub use window::Window;
pub use camera::FirstPerson;

pub mod utils;
mod camera;
mod renderer;
mod window;
