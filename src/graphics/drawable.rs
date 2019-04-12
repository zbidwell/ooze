use glium::{Frame, Program};
use glium::backend::glutin::Display;

pub trait Drawable {
    fn draw(&self, target: &mut Frame, display: &Display, program: &Program);
}
