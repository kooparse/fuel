use image;
use image::GenericImage;
use std::path::PathBuf;

const TEXTURE_ASSET_FOLDER: &str = "src/assets/textures";

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub raw_pixels: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub path: Box<PathBuf>,
}

impl Texture {
    pub fn new(texture_name: &str) -> Texture {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(TEXTURE_ASSET_FOLDER);
        path.push(texture_name);

        let img = image::open(&path).expect("Failed to load texture");
        let img_data = img.raw_pixels();

        Texture {
            id: 0,
            path: Box::new(path),
            raw_pixels: img_data,
            width: img.width() as i32,
            height: img.height() as i32,
        }
    }
}
