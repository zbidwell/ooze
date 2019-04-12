use image;
use std::io::Cursor;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use glium::Display;

pub struct Sprite {
    img_path: &'static str,
    texture: Texture2d,
}

impl Sprite {
    pub fn new(img_path: &'static str, display: &Display) -> Sprite {
        let image = image::load(Cursor::new(&include_bytes!("../res/a.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgb_reversed(&image.into_raw(), image_dimensions);

        Sprite {
            img_path,
            texture: Texture2d::new(display, image).unwrap()

        }
    }
}