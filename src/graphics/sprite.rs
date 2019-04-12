use image;
use std::io::BufReader;
use std::fs::File;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use glium::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpriteId {
    pub id: &'static str,
}

pub struct Sprite {
    pub texture: Texture2d,
}

impl Sprite {
    pub fn new(img_path: &str, display: &Display) -> Sprite {
        let r = BufReader::new(File::open(img_path).unwrap());
        let image = image::load(r, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        Sprite {
            texture: Texture2d::new(display, image).unwrap()
        }
    }
}