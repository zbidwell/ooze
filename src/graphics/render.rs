use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
glium::implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn from_arrays(position: [f32; 2], tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
}

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