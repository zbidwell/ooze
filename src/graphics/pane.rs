use crate::graphics::{Drawable, Terminal, Glyph};
use glium::Frame;
use glium::backend::glutin::Display;

use rand;

#[derive(PartialEq, Debug)]
pub struct Pane {
    pub glyph_width: usize,
    pub glyph_height: usize,

    pub width: usize,
    pub height: usize,    

    pub contents: Vec<Vec<Glyph>>,

    pub sub_panes: Vec<Pane>,
}

impl Pane {
    pub fn new(glyph_width: usize, glyph_height: usize, width: usize, height: usize) -> Pane {
        Pane {
            glyph_width,
            glyph_height,
            width,
            height,
            // 2D [x][y] Vec with capacity of [width][height]
            contents: {
                let mut outer = Vec::with_capacity(width as usize);
                for x in 0..width {
                    outer.push(Vec::with_capacity(height as usize));
                    for y in 0..height {
                        outer[x as usize].push(Glyph::empty_glyph(
                            [x, y],
                            glyph_width, // TODO: un-hardcode
                            glyph_height,
                        ));
                    }
                }
                outer
            },
            sub_panes: Vec::new(),
        }
    }

    pub fn add_sub_pane(&mut self, pane: Pane) {
        self.sub_panes.push(pane);
    }

    pub fn fill_with_random(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let color = [rand::random(), rand::random(), rand::random(), 1.0];
                self.contents[x as usize][y as usize] = Glyph::new(
                    color,
                    [x, y],
                    self.glyph_width, // TODO: un-hardcode
                    self.glyph_height,
                );
            }
        }
    }

    pub fn glyphs(&self) -> Vec<&Glyph> {
        let mut result = Vec::with_capacity(self.width * self.height);
        for column in &self.contents {
            for glyph in column {
                result.push(glyph);
            }
        }
        result
    }
}

impl Drawable for Pane {
    fn draw(&self, target: &mut Frame, display: &Display, terminal: &Terminal) {
        // draw all glyphs in pane
        for glyph in self.glyphs() {
            glyph.draw(target, display, terminal);
        }

        // draw all sub-panes too
        for pane in &self.sub_panes {
            pane.draw(target, display, terminal);
        }
    }
}
