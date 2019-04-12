use glium;
use glium::backend::glutin::Display;

use crate::graphics::{term_to_screen, Dimensions, Sprite, SpriteId};

pub struct Glyph {
    pub location: [usize; 2],
    
    pub dims: Dimensions,

    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub vertices: [Vertex; 4],
    pub sprite_id: SpriteId,
}

impl Glyph {
    pub fn new(fg_color: [f32; 4], bg_color: [f32; 4], sprite_id: SpriteId, location: [usize; 2], dims: Dimensions) -> Glyph {
        Glyph {
            location,
            dims,
            fg_color,
            bg_color,
            vertices: {
                let tl = term_to_screen([location[0], location[1] + 1], dims.term_width, dims.term_height);
                let tr = term_to_screen([location[0] + 1, location[1] + 1], dims.term_width, dims.term_height);
                let bl = term_to_screen([location[0], location[1]], dims.term_width, dims.term_height);
                let br = term_to_screen([location[0] + 1, location[1]], dims.term_width, dims.term_height);

                [
                    Vertex{position: tl, tex_coords: [0.0, 1.0]},
                    Vertex{position: tr, tex_coords: [1.0, 1.0]},
                    Vertex{position: bl, tex_coords: [0.0, 0.0]},
                    Vertex{position: br, tex_coords: [1.0, 0.0]},
                ]
            },
            sprite_id,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
glium::implement_vertex!(Vertex, position, tex_coords);

pub const V_SHADER: &str = r#"
    #version 140
            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
"#;

pub const F_SHADER: &str = r#"
    #version 140
            in vec2 v_tex_coords;

            out vec4 color;

            uniform vec4 bg_color;
            uniform vec4 fg_color;
            uniform sampler2D tex;

            void main() {
                if (texture(tex, v_tex_coords).a < 0.5) {
                    color = bg_color;
                } else {
                    color = fg_color;
                }
            }
"#;