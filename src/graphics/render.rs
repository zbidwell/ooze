use glium;
use std::fs::read_to_string;
use std::path::Path;

use crate::error::OozeResult;

/// A vertex for glium's rendering program.
#[derive(Copy, Clone)]
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
}

/// Return the shader source at the given path.
pub fn get_shader(path: &Path) -> OozeResult<String> {
    Ok(read_to_string(path)?)
}