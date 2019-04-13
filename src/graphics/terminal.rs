use std::collections::HashMap;
use crate::graphics::{Pane, Sprite, SpriteId, Dimensions};
use glium::{Frame, Program};
use glium::backend::glutin::Display;



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
            root_pane: Pane::new(dims, 0),
        }
    }

    pub fn draw(&self, target: &mut Frame, display: &Display, program: &Program, sprites: &HashMap<SpriteId, Sprite>) {
        self.root_pane.draw(target, display, self.dims, program, sprites);
    }
}



