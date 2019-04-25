use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{Read, BufReader};
use std::fs::{File};

use rusttype::{point, Font, Scale};
use image::{DynamicImage};

use glium::{Display};
use glium::texture::{Texture2d, RawImage2d};

pub struct TextureLibrary {
    pub display: Display,

    pub font_path: PathBuf,
    pub size: (u32, u32),

    pub map: HashMap<char, Texture2d>,
}

impl TextureLibrary {
    pub fn new<P: AsRef<Path>>(display: Display, font_path: P, size: (u32, u32)) -> TextureLibrary {
        TextureLibrary {
            display,
            map: HashMap::new(),
            font_path: {
                let mut p = PathBuf::new();
                p.push(font_path);
                p
            },
            size,
            }
    }

    pub fn build_string(&mut self, s: String) {
        for c in s.chars() {
            self.build(c);
        }
    }    
}

impl GenericTextureLibrary<char> for TextureLibrary {
    fn get(&self, c: char) -> &Texture2d {
        &self.map[&c]
    }

    fn build(&mut self, id: char) {
        let mut bytes = Vec::new();
        BufReader::new(File::open(&self.font_path).unwrap()).read_to_end(&mut bytes)
            .expect("Could not read file");
    
        let font_data = &bytes[..];
        let font = Font::from_bytes(font_data as &[u8])
            .expect("Could not create font from bytes");
    
        let (g_height, g_width) = self.size;

        let scale = Scale::uniform(g_height as f32);

        let glyph = font.glyph(id)
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

        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), self.size);

        let texture = Texture2d::new(&self.display, image)
            .expect("Could not create texture from image");

        self.map.insert(id, texture);
    }

    fn is_built(&self, id: char) -> bool {
        match self.map.get(&id) {
            Some(_) => false,
            None => true,
        }
    }
}

pub trait GenericTextureLibrary<T> {
    fn get(&self, id: T) -> &Texture2d;

    fn build(&mut self, id: T);

    fn is_built(&self, id: T) -> bool;
}