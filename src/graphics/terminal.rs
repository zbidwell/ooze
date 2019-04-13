use std::collections::HashMap;
use crate::graphics::{Pane, Sprite, SpriteId, Dimensions, Glyph, Point};
use glium::{Frame, Program, Surface, Blend};
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
            root_pane: Pane::new(dims),
        }
    }

    pub fn draw(&self, target: &mut Frame, display: &Display, program: &Program, sprites: &HashMap<SpriteId, Sprite>) {
        let glyph_tuples = self.collect_drawable_glyphs();

        let params = glium::DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        for (glyph, point, layer) in glyph_tuples {
            target.draw(
                &glium::VertexBuffer::new(display, &point.screen_verts(self.dims)).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                program,
                &glium::uniform!{bg_color: glyph.bg_color, fg_color: glyph.fg_color, tex: &sprites.get(&glyph.sprite_id).unwrap().texture},
                &params,
            ).unwrap();
        }
    }

    // collects vector of (Glyph, final_point, layer) from each subpane.
    fn collect_drawable_glyphs(&self) -> Vec<(&Glyph, Point, usize)> {
        let mut result: Vec<(&Glyph, Point, usize)> = Vec::new();
        let panes: Vec<&Pane> = self.root_pane.all_sub_panes();
        for pane in panes {
            if !pane.hidden {
                for glyph in pane.glyphs() {
                    result.push((&glyph, glyph.location.plus(pane.dims.offset), pane.layer));
                }
            }
        };
        result.sort_by_key(|(_, _, layer)| *layer);
        result
    }
}



