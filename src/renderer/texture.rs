use gl;
use gl::types::*;

use image;
use image::GenericImage;
use std::mem;
use std::os::raw::c_void;
use std::path::PathBuf;

const TEXTURE_ASSET_FOLDER: &str = "src/assets/textures";

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub raw_pixels: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub path: Box<PathBuf>,
    pub is_texture_activated: bool,
    stride: GLsizei,
}

impl Texture {
    pub fn new(texture_name: &str, stride: GLsizei) -> Texture {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(TEXTURE_ASSET_FOLDER);
        path.push(texture_name);

        let img = image::open(&path).expect("Failed to load texture");
        let img_data = img.raw_pixels();

        let mut texture = Texture {
            id: 0,
            stride,
            path: Box::new(path),
            raw_pixels: img_data,
            width: img.width() as i32,
            height: img.height() as i32,
            is_texture_activated: true,
        };

        texture.set_texture();
        texture
    }

    pub fn set_texture(&mut self) {
        unsafe { self.setup() }
    }

    pub fn set_texture_off(&mut self) {
        self.is_texture_activated = false
    }

    pub fn set_texture_true(&mut self) {
        self.is_texture_activated = true
    }

    // Draw our stuff
    pub fn render(&mut self) {
        let mut texture_id: u32 = 0;

        if self.is_texture_activated {
            texture_id = self.id;
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }
}

impl TextureSetup for Texture {
    // Setup our texture with openGL
    unsafe fn setup(&mut self) {
        let mut texture_id = self.id;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            self.stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as i32,
        );

        // Texture filter paramenters
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as i32,
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_LINEAR as i32,
        );

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            self.width,
            self.height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &self.raw_pixels[0] as *const u8 as *const c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
        self.id = texture_id;
    }
}

pub trait TextureSetup {
    unsafe fn setup(&mut self);
}
