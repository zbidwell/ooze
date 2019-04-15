use glium;
use std::fs::read_to_string;
use lazy_static::lazy_static;

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

// needed to load and export things. Kind of like a const that is created on first access?
lazy_static! {
    pub static ref V_SHADER: String = read_to_string(r#"resources\shaders\vertex\v_shader_default.vert"#).unwrap();
    pub static ref F_SHADER: String = read_to_string(r#"resources\shaders\fragment\f_shader_default.frag"#).unwrap();
}