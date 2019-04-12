use glium;
use glium::Frame;
use glium::Surface;
use glium::backend::glutin::Display;

use crate::graphics::{Drawable, Terminal};
use crate::app::App;

#[derive(PartialEq, Debug)]
pub struct Glyph {
    pub location: [usize; 2],
    pub width: usize,
    pub height: usize,

    pub color: [f32; 4],
}

impl Glyph {
    pub fn new(color: [f32; 4], location: [usize; 2], width: usize, height:usize) -> Glyph {
        Glyph {
            location,
            width,
            height,
            color
        }
    }

    pub fn empty_glyph(location: [usize; 2], width: usize, height:usize) -> Glyph {
        Glyph {
            location,
            width,
            height,
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Drawable for Glyph {
    fn draw(&self, target: &mut Frame, display: &Display, terminal: &Terminal) {
        let tl = terminal.term_to_screen([self.location[0], self.location[1] + 1]);
        let tr = terminal.term_to_screen([self.location[0] + 1, self.location[1] + 1]);
        let bl = terminal.term_to_screen([self.location[0], self.location[1]]);
        let br = terminal.term_to_screen([self.location[0] + 1, self.location[1]]);

        let vertices = [
            Vertex{position: tl},
            Vertex{position: tr},
            Vertex{position: bl},
            Vertex{position: br},
        ];

        let v_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();

        let program = 
            glium::Program::from_source(display, V_SHADER, F_SHADER, None).unwrap();

        target.draw(
            &v_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &program,
            &glium::uniform!{quad_color: self.color},
            &glium::DrawParameters::default()
        ).unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

pub const V_SHADER: &str = r#"
    #version 140
            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
"#;

pub const F_SHADER: &str = r#"
    #version 140
            out vec4 color;

            uniform vec4 quad_color;

            void main() {
                color = quad_color;
            }
"#;