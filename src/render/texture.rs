use std::collections::HashMap;
use std::path::Path;
use std::io::{Read, BufReader};
use std::fs::{File};

use rusttype::{point, Font, Scale};
use image::{DynamicImage};

use glium::{Display};
use glium::texture::{Texture2d, RawImage2d};

pub struct TextureLibrary {
    pub display: Display,

    pub map: HashMap<char, Texture2d>,
}

impl TextureLibrary {
    pub fn new(display: Display) -> TextureLibrary {
        TextureLibrary { display, map: HashMap::new() }
    }

    pub fn build_string<P: AsRef<Path>>(&mut self, s: String, font: P, size: (u32, u32)) {
        for c in s.chars() {
            self.build(c, &font, size);
        }
    }

    pub fn build<P: AsRef<Path>>(&mut self, c: char, font: P, size: (u32, u32)) {
        let mut bytes = Vec::new();
        BufReader::new(File::open(font).unwrap()).read_to_end(&mut bytes)
            .expect("Could not read file");
    
        let font_data = &bytes[..];
        let font = Font::from_bytes(font_data as &[u8])
            .expect("Could not create font from bytes");
    
        let (g_height, g_width) = size;

        let scale = Scale::uniform(g_height as f32);

        let glyph = font.glyph(c)
            .scaled(scale)
            .positioned(point(0.0, 0.0));

        let mut image = DynamicImage::new_rgba8(g_width as u32, g_height as u32).to_rgba();
    
        glyph.draw(|x, y, v| {
            image.put_pixel(
                x + ((g_width - glyph.pixel_bounding_box().unwrap().width() as u32) / 2),
                y + ((g_height - glyph.pixel_bounding_box().unwrap().height() as u32) / 2),
                image::Rgba {
                    data: [0, 0, 0, (v * 255.0) as u8]
                }
            )
        });

        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), size);

        let texture = Texture2d::new(&self.display, image)
            .expect("Could not create texture from image");

        self.map.insert(c, texture);
    }

    pub fn get(&self, c: char) -> &Texture2d {
        self.map.get(&c).unwrap()
    }
}