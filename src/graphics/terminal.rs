use crate::graphics::Drawable;
use crate::graphics::Pane;
use glium::{Frame, Program};
use glium::backend::glutin::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub glyph_width: usize,
    pub glyph_height: usize,
    pub term_width: usize,
    pub term_height: usize,
}

impl Dimensions {
    pub fn new(glyph_width: usize, glyph_height: usize, term_width: usize, term_height: usize) -> Dimensions {
        Dimensions {
            glyph_width,
            glyph_height,
            term_width,
            term_height,
        }
    }
}

pub struct Terminal {
    pub dims: Dimensions,

    pub root_pane: Pane,
}

impl Terminal {
    pub fn new(
        dims: Dimensions,
    ) -> Terminal {
        Terminal {
            dims,
            root_pane: Pane::new(dims),
        }
    }

    pub fn term_to_screen(&self, location: [usize; 2]) -> [f32; 2] {
        [
            2.0 * ((location[0] as f32) / self.dims.term_width as f32) - 1.0,
            2.0 * ((location[1] as f32) / self.dims.term_height as f32) - 1.0
        ]
    }
}

impl Drawable for Terminal {
    fn draw(&self, target: &mut Frame, display: &Display, program: &Program) {
        self.root_pane.draw(target, display, program);
    }
}

pub fn term_to_screen(location: [usize; 2], term_width: usize, term_height: usize) -> [f32; 2] {
    [
        2.0 * ((location[0] as f32) / term_width as f32) - 1.0,
        2.0 * ((location[1] as f32) / term_height as f32) - 1.0
    ]
}


