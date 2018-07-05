use gl;
use gl::types::*;
use gltf;
use gltf::image::Source;

use image;
use image::GenericImage;
use image::ImageFormat::{JPEG, PNG};
use std::mem;
use std::os::raw::c_void;
use std::path::PathBuf;

const TEXTURE_ASSET_FOLDER: &str = "src/assets/textures";

#[derive(Clone, Debug, Default)]
pub struct Texture {
    pub glTF_index: usize,
    pub name: Option<String>,
    pub gl_id: u32,
    pub tex_coord: u32,
}

impl Texture {
    pub fn from_gltf(
        gltf_texture: gltf::Texture,
        buffers: &Vec<gltf::buffer::Data>,
    ) {
        let glTF_index = gltf_texture.index();
        let data = match gltf_texture.source().source() {
            Source::View { view, mime_type } => {
                let img = match mime_type {
                    "image/jpeg" => {
                        // let data = view.buffer().source();
                    }
                    "image/png" => {
                        // image::load_from_memory_with_format(view.buffer(), PNG)
                    }
                    _ => println!("prob"),
                };
            }
            Source::Uri { uri, .. } => println!("Uri not supported."),
            _ => println!("Texture format not supported."),
        };

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        Texture {
            ..Default::default()
        };
    }
}

// impl Texture {
//     #[allow(dead_code)]
//     pub fn new(texture_name: &str, stride: GLsizei) -> Texture {
//         let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         path.push(TEXTURE_ASSET_FOLDER);
//         path.push(texture_name);

//         let img = image::open(&path).expect("Failed to load texture");
//         let img_data = img.raw_pixels();

//         let mut texture = Texture {
//             id: 0,
//             stride,
//             path: Box::new(path),
//             raw_pixels: img_data,
//             width: img.width() as i32,
//             height: img.height() as i32,
//             is_texture_activated: true,
//         };

//         texture.set_texture();
//         texture
//     }

//     #[allow(dead_code)]
//     pub fn set_texture(&mut self) {
//         unsafe { self.setup() }
//     }

//     #[allow(dead_code)]
//     pub fn set_texture_off(&mut self) {
//         self.is_texture_activated = false
//     }

//     #[allow(dead_code)]
//     pub fn set_texture_true(&mut self) {
//         self.is_texture_activated = true
//     }

//     #[allow(dead_code)]
//     // Draw our stuff
//     pub fn render(&mut self) {
//         let mut texture_id: u32 = 0;

//         if self.is_texture_activated {
//             texture_id = self.id;
//         };

//         unsafe {
//             gl::BindTexture(gl::TEXTURE_2D, texture_id);
//         }
//     }
// }

// impl TextureSetup for Texture {
//     // Setup our texture with openGL
//     unsafe fn setup(&mut self) {
//         let mut texture_id = self.id;
//         gl::GenTextures(1, &mut texture_id);
//         gl::BindTexture(gl::TEXTURE_2D, texture_id);

//         gl::VertexAttribPointer(
//             1,
//             2,
//             gl::FLOAT,
//             gl::FALSE,
//             self.stride,
//             (3 * mem::size_of::<GLfloat>()) as *const c_void,
//         );
//         gl::EnableVertexAttribArray(1);

//         gl::TexParameteri(
//             gl::TEXTURE_2D,
//             gl::TEXTURE_WRAP_S,
//             gl::MIRRORED_REPEAT as i32,
//         );

//         gl::TexParameteri(
//             gl::TEXTURE_2D,
//             gl::TEXTURE_WRAP_T,
//             gl::MIRRORED_REPEAT as i32,
//         );

//         // Texture filter paramenters
//         gl::TexParameteri(
//             gl::TEXTURE_2D,
//             gl::TEXTURE_MIN_FILTER,
//             gl::NEAREST as i32,
//         );

//         gl::TexParameteri(
//             gl::TEXTURE_2D,
//             gl::TEXTURE_MIN_FILTER,
//             gl::LINEAR as i32,
//         );

//         gl::TexParameteri(
//             gl::TEXTURE_2D,
//             gl::TEXTURE_MIN_FILTER,
//             gl::NEAREST_MIPMAP_LINEAR as i32,
//         );

//         gl::TexImage2D(
//             gl::TEXTURE_2D,
//             0,
//             gl::RGB as i32,
//             self.width,
//             self.height,
//             0,
//             gl::RGB,
//             gl::UNSIGNED_BYTE,
//             &self.raw_pixels[0] as *const u8 as *const c_void,
//         );

//         gl::GenerateMipmap(gl::TEXTURE_2D);
//         self.id = texture_id;
//     }
// }

// pub trait TextureSetup {
//     unsafe fn setup(&mut self);
// }
