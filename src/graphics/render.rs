use glium;
use std::fs::read_to_string;
use std::path::Path;

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

pub fn get_shader(path: &Path) -> String {
    read_to_string(path).unwrap()
}