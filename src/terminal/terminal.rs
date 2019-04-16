use crate::app::OozeResult;
use crate::terminal::{Pane, Glyph};
use crate::geometry::{Dimensions, Point};
use crate::graphics::{SpriteMap};
use glium::{Frame, Program, Surface, Blend};
use glium::backend::glutin::Display;
use glium::uniforms::Sampler;
use glium::uniforms::MagnifySamplerFilter::Nearest;

pub struct Terminal {
    pub dims: Dimensions,

    pub root_pane: Pane,
}

impl Terminal {
    pub fn new(dims: Dimensions) -> OozeResult<Terminal> {
        let terminal = Terminal {
            dims,
            root_pane: Pane::new(dims)?,
        };

        Ok(terminal)
    }

    pub fn draw(&self, target: &mut Frame, display: &Display, program: &Program, sprites: &SpriteMap) -> OozeResult<()> {
        let glyph_tuples = self.collect_drawable_glyphs();

        let params = glium::DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        for (glyph, point, _layer) in glyph_tuples {
            let texture = &sprites.get(&glyph.sprite_id)?.texture;

            let uniforms = glium::uniform! {
                bg_color: glyph.bg_color,
                fg_color: glyph.fg_color,
                tex: Sampler::new(texture).magnify_filter(Nearest)
            };

            target.draw(
                &glium::VertexBuffer::new(display, &point.screen_verts(self.dims)).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                program,
                &uniforms,
                &params,
            ).unwrap();
        }

        Ok(())
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



