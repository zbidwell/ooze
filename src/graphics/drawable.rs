use glium::Frame;
use glium::backend::glutin::Display;
use crate::graphics::Terminal;

pub trait Drawable {
    fn draw(&self, target: &mut Frame, display: &Display, terminal: &Terminal);
}
