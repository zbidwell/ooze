use std::collections::HashMap;
use crate::graphics::{Terminal, Glyph, V_SHADER, F_SHADER, Vertex, Dimensions, Sprite, Point};
use glium::{Frame, Surface, Program, Blend};
use glium::backend::glutin::Display;

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
                            [1.0, 1.0, 1.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
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

    pub fn add_sub_pane_with(&mut self, dims: Dimensions){
        let mut pane = Pane::new(dims);
        pane.layer = self.layer + 1;
        self.add_sub_pane(pane);
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
