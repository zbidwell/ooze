use glium::{Display, Surface, Program, glutin, Blend, uniform};
use glium::texture::{Texture2d};
use glium::uniforms::Sampler;
use glium::uniforms::MagnifySamplerFilter::Nearest;

use std::fs::{read_to_string};
use std::time::Instant;
use std::path::Path;

use crate::color::*;

/// A vertex for glium's rendering program.
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
glium::implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    /// Build a vertex from position and texture coordinate arrays.
    pub fn from_arrays(position: [f32; 2], tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }

    pub fn from_cell(x: usize, y: usize, max_x: usize, max_y: usize, tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position: {
                [((x as f32 / max_x as f32) - 0.5) * 2.0, ((y as f32 / max_y as f32) - 0.5) * 2.0]
            },
            tex_coords,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Glyph<'a> {
    texture: &'a Texture2d,

    program: &'a Program,

    fg_color: Color,
    bg_color: Color,
}

impl<'a> Glyph<'a> {
    pub fn new(texture: &'a Texture2d, program: &'a Program) -> Glyph<'a> {
        Glyph {
            texture,
            program,
            fg_color: WHITE,
            bg_color: CLEAR,
        }
    }

    pub fn new_ex(texture: &'a Texture2d, program: &'a Program, fg_color: Color, bg_color: Color) -> Glyph<'a> {
        Glyph {
            texture,
            program,
            fg_color,
            bg_color,
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

    pub size: (usize, usize, usize),
    pub contents: Vec<Vec<Vec<Option<Glyph<'a>>>>>,

    dirties: Vec<(usize, usize)>,
    old_dirties: Vec<(usize, usize)>,
}

impl<'a> Terminal<'a> {
    pub fn new(display: Display, width: usize, height: usize, layers: usize) -> Terminal<'a> {
        Terminal {
            display,
            size: (width, height, layers),
            contents: {
                let mut v = Vec::new();
                for x in 0..width {
                    v.push(Vec::new());
                    for y in 0..height {
                        v[x].push(Vec::new());
                        for _l in 0..layers {
                            v[x][y].push(None);
                        }
                    }
                }
                v
            },
            dirties: Vec::new(),
            old_dirties: Vec::new(),
        }
    }

    fn get_points_2d(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                v.push((x, y))
            }
        }
        v
    }

    pub fn set_empty(&mut self, x: usize, y: usize, layer: usize) {
        self.contents[x][y][layer] = None;
        self.dirties.push((x, y));
    }

    pub fn clear_dirties(&mut self) {
        self.old_dirties = self.dirties.clone();
        self.dirties = Vec::new();
    }

    pub fn fill_with(&mut self, layer: usize, glyph: Glyph<'a>) {
        for (x, y) in self.get_points_2d() {
            self.set(x, y, layer, glyph.clone());
        }
    }

    pub fn make_border(&mut self, layer: usize, glyph: Glyph<'a>) {
        for (x, y) in self.get_points_2d() {
            if (x == 0 || x == self.size.0 - 1) || (y == 0 || y == self.size.1 - 1) {
                self.set(x, y, layer, glyph.clone());
            }
        } 
    }

    pub fn clear_layer(&mut self, layer: usize) {
        for (x, y) in self.get_points_2d() {
            self.contents[x][y][layer] = None;
            self.dirties.push((x, y));
        }
    }
}

impl<'a> Renderer<Glyph<'a>> for Terminal<'a> {
    fn size(&self) -> (usize, usize, usize) {
        self.size
    }

    fn set(&mut self, x: usize, y: usize, layer: usize, renderable: Glyph<'a>) {
        self.contents[x][y][layer] = Some(renderable);
        self.dirties.push((x, y));
    }

    fn get(&self, x: usize, y: usize, layer: usize) -> &Option<Glyph<'a>> {
        &self.contents[x][y][layer]
    }

    fn get_all(&self) -> Vec<&Option<Glyph<'a>>> {
        let mut v = Vec::new();
        for (x, y, l) in self.get_points() {
            v.push(&self.contents[x][y][l]);
        }
        v
    }

    fn get_points(&self) -> Vec<(usize, usize, usize)> {
        let mut v = Vec::new();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                for l in 0..self.size.2 {
                    v.push((x, y, l));
                }
            }
        }
        v
    }

    fn clear(&mut self) {
        for (x, y) in self.get_points_2d() {
            for l in 0..self.size.2 {
                self.contents[x][y][l] = None;
            }
            self.dirties.push((x, y));
        }
    }

    fn render(&mut self) {
        //let start = Instant::now();
        // create frame
        let mut target = self.display.draw();

        let mut v: Vec<(&Glyph, [Vertex; 4], usize)> = Vec::new();
        // double buffered, so we need changes for both this frame and last frame
        for (x, y) in self.dirties.iter().chain(self.old_dirties.iter()) {
            for l in 0..self.size.2 {
                if let Some(g) = self.get(*x, *y, l) {
                    let verts = [
                        Vertex::from_cell(*x, y + 1, self.size.0, self.size.1, [0.0, 1.0]),
                        Vertex::from_cell(x + 1, y + 1, self.size.0, self.size.1, [1.0, 1.0]),
                        Vertex::from_cell(*x, *y, self.size.0, self.size.1, [0.0, 0.0]),
                        Vertex::from_cell(x + 1, *y, self.size.0, self.size.1, [1.0, 0.0]),
                    ];
                    v.push((g, verts, l));
                }
            }
        }
        v.sort_by_key(|(_, _, layer)| *layer);

        // draw the Glyphs back-to-front
        let params = glium::DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        for (glyph, verts, _layer) in v {
            let texture = glyph.texture();
            
            let uniforms = uniform! {
                bg_color: glyph.bg_color.as_array(),
                fg_color: glyph.fg_color.as_array(),
                tex: Sampler::new(texture).magnify_filter(Nearest),
            };

            target.draw(
                &glium::VertexBuffer::new(&self.display, &verts).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                glyph.program(),
                &uniforms,
                &params,
            ).unwrap();
        }

        target.finish().unwrap();

        //println!("{:?}", start.elapsed());

        self.clear_dirties();
    }
}

pub trait Renderer<R: Renderable> {
    /// width, height, layers
    fn size(&self) -> (usize, usize, usize);

    fn set(&mut self, x: usize, y: usize, layer: usize, renderable: R);

    fn get(&self, x: usize, y: usize, layer: usize) -> &Option<R>;

    fn get_all(&self) -> Vec<&Option<R>>;

    fn get_points(&self) -> Vec<(usize, usize, usize)>;

    fn clear(&mut self);

    fn render(&mut self);
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

pub fn build_program<P: AsRef<Path>>(display: &Display, v_shader_path: P, f_shader_path: P) -> Program {
    Program::from_source(
        display,
        read_to_string(v_shader_path).unwrap().as_str(),
        read_to_string(f_shader_path).unwrap().as_str(),
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