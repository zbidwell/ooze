use std::collections::HashMap;
use glium::{Frame, Program};
use glium::backend::glutin::Display;
use crate::graphics::{Sprite, SpriteId};

pub trait Drawable {
    fn draw(&self, target: &mut Frame, display: &Display, program: &Program, sprites: &HashMap<SpriteId, Sprite>);
}
