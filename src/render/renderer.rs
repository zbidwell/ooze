use glium;
use glium::{Display, Frame, Program, glutin};
use glium::texture::{Texture2d};

use std::fs::{read_to_string};

#[derive(Debug)]
pub struct Glyph<'a> {
    texture: &'a Texture2d,

    program: &'a Program,

    fg_color: [f32; 4],
    bg_color: [f32; 4],
}

impl<'a> Glyph<'a> {
    pub fn new(texture: &'a Texture2d, program: &'a Program) -> Glyph<'a> {
        Glyph {
            texture,
            program,
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 0.0],
        }
    }

    
}

impl Renderable for Glyph<'_> {
    fn texture(&self) -> &Texture2d {
        &self.texture
    }

    fn program(&self) -> &Program {
        &self.program
    }
}

pub struct Terminal<'a> {
    display: Display,

    size: (usize, usize, usize),
    cell_size: (usize, usize),
    contents: Vec<Vec<Vec<Option<Glyph<'a>>>>>,
}

impl<'a> Terminal<'a> {
    pub fn new(display: Display, width: usize, height: usize, layers: usize, cell_width: usize, cell_height: usize) -> Terminal<'a> {
        Terminal {
            display,
            size: (width, height, layers),
            cell_size: (cell_width, cell_height),
            contents: {
                let mut v = Vec::new();
                for x in 0..width {
                    v.push(Vec::new());
                    for y in 0..height {
                        v[x].push(Vec::new());
                        for l in 0..layers {
                            v[x][y].push(None);
                        }
                    }
                }
                v
            }
        }
    }
}

impl<'a> Renderer<Glyph<'a>> for Terminal<'a> {
    fn size(&self) -> (usize, usize, usize) {
        self.size
    }

    fn set(&mut self, x: usize, y: usize, layer: usize, renderable: Glyph<'a>) {
        self.contents[x][y][layer] = Some(renderable);
    }

    fn get(&mut self, x: usize, y: usize, layer: usize) -> &Option<Glyph<'a>> {
        &self.contents[x][y][layer]
    }

    fn get_all(&self) -> Vec<&Option<Glyph<'a>>> {
        let mut v = Vec::new();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                for l in 0..self.size.2 {
                    v.push(&self.contents[x][y][l]);
                }
            }
        }
        v
    }

    fn clear(&mut self) {
        for column in self.contents.iter_mut() {
            for row in column {
                for layer in row {
                    *layer = None
                }
            }
        }
    }

    fn render(&self) {
        for g in self.get_all() {
            println!("{:?}", g);
        }
    }
}

pub trait Renderer<R: Renderable> {
    /// width, height, layers
    fn size(&self) -> (usize, usize, usize);

    fn set(&mut self, x: usize, y: usize, layer: usize, renderable: R);

    fn get(&mut self, x: usize, y: usize, layer: usize) -> &Option<R>;

    fn get_all(&self) -> Vec<&Option<R>>;

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
        read_to_string("./resources/shaders/fragment/f_shader_default.frag").unwrap().as_str(),
        None,
    ).expect("Failed to create shader program")
}

pub fn init_window(width: usize, height: usize, title: &str) -> (glutin::EventsLoop, glium::Display) {
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

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_texture_library_build() {
//         let (_, d) = init_window(100, 100, "test");

//         let mut tl = TextureLibrary::new(d.clone());

//         tl.build_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(), "./resources/fonts/Roboto-Regular.ttf", (32, 32));

//         tl.get('a');
//         tl.get('A');
//     }
// }