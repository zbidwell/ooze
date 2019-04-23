use glium::{Display, Frame, Program, glutin};
use glium::texture::{Texture2d, RawImage2d};

use rusttype::{point, Font, Scale};
use image::{ImageBuffer, DynamicImage, Rgba};

use std::fs::{read_to_string, File};
use std::io::{BufReader, Read};
use std::collections::HashMap;
use std::path::Path;

pub trait Renderer<R: Renderable> {
    /// width, height, layers
    fn size(&self) -> (usize, usize, usize);

    fn set(&mut self, x: usize, y: usize, layer: usize, renderable: R);

    fn get(&mut self, x: usize, y: usize, layer: usize) -> &Option<R>;

    fn clear(&mut self);

    fn render(&self);
}


pub trait Renderable {
    fn texture(&self) -> &Texture2d;

    fn program(&self) -> &Program;
}


pub fn default_program(display: &Display) -> Program {
    Program::from_source(
        display,
        read_to_string("./resources/shaders/vertex/v_shader_default.vert").unwrap().as_str(),
        read_to_string("./resources/shaders/fragment/f_shader_default.vert").unwrap().as_str(),
        None,
    ).expect("Failed to create shader program")
}

#[derive(Debug)]
pub struct TextureLibrary {
    map: HashMap<char, Texture2d>,
}

impl TextureLibrary {
    pub fn new() -> TextureLibrary {
        TextureLibrary { map: HashMap::new() }
    }

    pub fn build_string<P: AsRef<Path>>(&mut self, display: &Display, s: String, font: P, size: (u32, u32)) {
        for c in s.chars() {
            self.build(display, c, &font, size);
        }
    }

    pub fn build<P: AsRef<Path>>(&mut self, display: &Display, c: char, font: P, size: (u32, u32)) {
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

        let texture = Texture2d::new(display, image)
            .expect("Could not create texture from image");

        self.map.insert(c, texture);
    }

    pub fn get(&self, c: char) -> &Texture2d {
        self.map.get(&c).unwrap()
    }

    
}

fn init_window(width: usize, height: usize, title: &str) -> (glutin::EventsLoop, glium::Display) {
    let size = glutin::dpi::LogicalSize::new(width as f64, height as f64);

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(size)
        .with_resizable(false)
        .with_title(title);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop)
        .expect("Could not create window");

    (events_loop, display)
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct SimpleRenderable {
//     pub val: char,
// }

// impl Renderable for SimpleRenderable {
//     fn texture(&self) -> char {
//         self.val
//     }
// }

// pub struct SimpleRenderer<SimpleRenderable> {
//     //pub display: &'a Display,

//     pub contents: [[Option<SimpleRenderable>; 30]; 50],
// }

// impl SimpleRenderer<SimpleRenderable> {
//     pub fn new() -> SimpleRenderer<SimpleRenderable> {
//         let contents = [[None::<SimpleRenderable>; 30]; 50];
//         SimpleRenderer { contents }
//     }
// }

// impl Renderer<SimpleRenderable> for SimpleRenderer<SimpleRenderable> {
//     fn view_size(&self) -> (usize, usize, usize) {
//         (30, 50 , 1)
//     }

//     fn set(&mut self, x: usize, y: usize, _layer: usize, renderable: SimpleRenderable) {
//         self.contents[x][y] = Some(renderable)
//     }

//     fn get(&mut self, x: usize, y: usize, _layer: usize) -> &Option<SimpleRenderable> {
//         &self.contents[x][y]
//     }

//     fn clear(&mut self) {
//         for row in self.contents.iter_mut() {
//             for val in row {
//                 *val = None;
//             }
//         }
//     }

//     fn render(&self) {
//         for row in self.contents.iter() {
//             for val in row {
//                 println!("{:?}", val);
//             }
//         } 
//     }
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_texture_library_build() {
        let (_, d) = init_window(100, 100, "test");

        let mut tl = TextureLibrary::new();

        tl.build_string(&d, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), "./resources/fonts/Roboto-Regular.ttf", (32, 32));

        tl.get('a');
        tl.get('A');
    }
}
//     #[test]
//     fn test_set() {
//         let (mut sr, r) = setup();

//         sr.set(3, 4, 1, r);
//     }

//     #[test]
//     fn test_get() {
//         let (mut sr, r) = setup();

//         sr.set(3, 4, 1, r);
//         assert_eq!(sr.get(3, 4, 1), &Some(r));
//     }

//     #[test]
//     fn test_clear() {
//         let (mut sr, r) = setup();

//         sr.set(3, 4, 1, r);
//         sr.clear();
//         assert_eq!(sr.get(3, 4, 1), &None);
//     }
// }