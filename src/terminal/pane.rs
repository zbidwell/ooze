use crate::terminal::{Glyph};
use crate::graphics::{Dimensions, Point};

use rand;

pub struct Pane {
    pub dims: Dimensions,

    pub layer: usize,
    pub hidden: bool,

    pub contents: Vec<Vec<Glyph>>,

    pub sub_panes: Vec<Pane>,
}

impl Pane {
    pub fn new(dims: Dimensions) -> Pane {
        Pane {
            dims,
            layer: 0,
            hidden: false,
            // 2D [x][y] Vec with capacity of [width][height]
            contents: {
                let mut outer = Vec::with_capacity(dims.term_size.x as usize);
                for x in 0..dims.term_size.x {
                    outer.push(Vec::with_capacity(dims.term_size.y as usize));
                    for y in 0..dims.term_size.y {
                        outer[x as usize].push(Glyph::new(
                            Point::new(x, y),
                            [1.0, 1.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 0.0],
                            "empty".to_string(),
                        ));
                    }
                }
                outer
            },
            sub_panes: Vec::new(),
        }
    }

    pub fn hide(&mut self) {
        if !self.hidden {
            self.hidden = true;
        }
    }

    pub fn show(&mut self) {
        if self.hidden {
            self.hidden = false;
        }
    }

    pub fn add_sub_pane(&mut self, mut pane: Pane) {
        pane.dims.offset = pane.dims.offset.plus(self.dims.offset);
        pane.layer = self.layer + 1;
        self.sub_panes.push(pane);
    }

    pub fn add_sub_pane_with(&mut self, dims: Dimensions) {
        let mut pane = Pane::new(dims);
        self.add_sub_pane(pane);
    }

    pub fn place(&mut self, x: usize, y: usize, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) {
        self.contents[x as usize][y as usize] = Glyph::new(
            Point::new(x as i32, y as i32),
            fg_color,
            bg_color,
            id.to_string(),
        );
    }

    pub fn make_border(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) {
        for x in 0..self.dims.term_size.x {
            for y in 0..self.dims.term_size.y {
                if x == 0 || x == self.dims.term_size.x - 1 || y == 0 || y == self.dims.term_size.y - 1 {
                    self.contents[x as usize][y as usize] = Glyph::new(
                        Point::new(x, y),
                        fg_color,
                        bg_color,
                        id.to_string(),
                    );
                }
            }
        }
    }

    pub fn fill_with(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) {
        for x in 0..self.dims.term_size.x {
            for y in 0..self.dims.term_size.y {
                self.contents[x as usize][y as usize] = Glyph::new(
                    Point::new(x, y),
                    fg_color,
                    bg_color,
                    id.to_string(),
                );
            }
        }
    }

    pub fn fill_with_random(&mut self) {
        for x in 0..self.dims.term_size.x {
            for y in 0..self.dims.term_size.y {
                let fg_color = [rand::random(), rand::random(), rand::random(), 1.0];
                let bg_color = [rand::random(), rand::random(), rand::random(), 1.0];
                self.contents[x as usize][y as usize] = Glyph::new(
                    Point::new(x, y),
                    fg_color,
                    bg_color,
                    "@".to_string(),
                );
            }
        }
    }

    pub fn glyphs(&self) -> Vec<&Glyph> {
        let mut result = Vec::with_capacity((self.dims.term_size.x * self.dims.term_size.y) as usize);
        for column in &self.contents {
            for glyph in column {
                result.push(glyph);
            }
        }
        result
    }

    pub fn all_sub_panes(&self) -> Vec<&Pane> {
        let mut result: Vec<&Pane> = Vec::new();
        result.push(&self);
        for pane in &self.sub_panes {
            result.extend(pane.all_sub_panes().iter());
        };

        result
    }
}
