use std::collections::HashMap;
use crate::graphics::{Drawable, Terminal, Glyph, V_SHADER, F_SHADER, Vertex, Dimensions, SpriteId, Sprite};
use glium::{Frame, Surface, Program, Blend};
use glium::backend::glutin::Display;

use rand;

pub struct Pane {
    pub dims: Dimensions,    

    pub contents: Vec<Vec<Glyph>>,

    pub sub_panes: Vec<Pane>,
}

impl Pane {
    pub fn new(dims: Dimensions) -> Pane {
        Pane {
            dims,
            // 2D [x][y] Vec with capacity of [width][height]
            contents: {
                let mut outer = Vec::with_capacity(dims.term_width as usize);
                for x in 0..dims.term_width {
                    outer.push(Vec::with_capacity(dims.term_height as usize));
                    for y in 0..dims.term_height {
                        outer[x as usize].push(Glyph::new(
                            [0.0, 0.0, 0.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                            SpriteId{id:"empty"},
                            [x, y],
                            dims
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
        for x in 0..self.dims.term_width {
            for y in 0..self.dims.term_height {
                let fg_color = [rand::random(), rand::random(), rand::random(), 1.0];
                let bg_color = [rand::random(), rand::random(), rand::random(), 1.0];
                self.contents[x as usize][y as usize] = Glyph::new(
                    fg_color,
                    bg_color,
                    SpriteId{id:"a"},
                    [x, y],
                    self.dims
                );
            }
        }
    }

    pub fn glyphs(&self) -> Vec<&Glyph> {
        let mut result = Vec::with_capacity(self.dims.term_width * self.dims.term_height);
        for column in &self.contents {
            for glyph in column {
                result.push(glyph);
            }
        }
        result
    }
}

impl Drawable for Pane {
    fn draw(&self, target: &mut Frame, display: &Display, program: &Program, sprites: &HashMap<SpriteId, Sprite>) {
        // draw all glyphs in pane
        let params = glium::DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        for glyph in self.glyphs() {
            // draw background
            target.draw(
                &glium::VertexBuffer::new(display, &glyph.vertices).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                program,
                &glium::uniform!{bg_color: glyph.bg_color, fg_color: glyph.fg_color, tex: &sprites.get(&glyph.sprite_id).unwrap().texture},
                &params,
            ).unwrap();
            // draw sprite
            {};
        }

        // draw all sub-panes too
        for pane in &self.sub_panes {
            pane.draw(target, display, program, sprites);
        }
    }
}
