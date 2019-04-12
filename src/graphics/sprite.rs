use image;
use std::io::BufReader;
use std::fs::File;
use glium::texture::RawImage2d;
use glium::texture::CompressedSrgbTexture2d;
use glium::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpriteId {
    pub id: usize,
}

pub struct Sprite {
    pub texture: CompressedSrgbTexture2d,
}

impl Sprite {
    pub fn new(img_path: &str, display: &Display) -> Sprite {
        let r = BufReader::new(File::open(img_path).unwrap());
        let image = image::load(r, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        // let image = image::load(Cursor::new(&include_bytes!("../res/a.png")[..]),
        //                         image::PNG).unwrap().to_rgba();
        // let image_dimensions = image.dimensions();
        // let image = RawImage2d::from_raw_rgb_reversed(&image.into_raw(), image_dimensions);

        // println!("{:?}", image);

        Sprite {
            texture: CompressedSrgbTexture2d::new(display, image).unwrap()
        }
    }
}