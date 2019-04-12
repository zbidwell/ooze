use glium;
use glium::backend::glutin::Display;

use crate::graphics::{term_to_screen, Dimensions, Sprite};

pub struct Glyph {
    pub location: [usize; 2],
    
    pub dims: Dimensions,

    pub color: [f32; 4],
    pub vertices: [Vertex; 4],
}

impl Glyph {
    pub fn new(color: [f32; 4], location: [usize; 2], dims: Dimensions) -> Glyph {
        Glyph {
            location,
            dims,
            color,
            vertices: {
                let tl = term_to_screen([location[0], location[1] + 1], dims.term_width, dims.term_height);
                let tr = term_to_screen([location[0] + 1, location[1] + 1], dims.term_width, dims.term_height);
                let bl = term_to_screen([location[0], location[1]], dims.term_width, dims.term_height);
                let br = term_to_screen([location[0] + 1, location[1]], dims.term_width, dims.term_height);

                [
                    Vertex{position: tl},
                    Vertex{position: tr},
                    Vertex{position: bl},
                    Vertex{position: br},
                ]
            },
        }
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