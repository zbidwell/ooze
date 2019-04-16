use crate::app::OozeResult;
use crate::terminal::{Panel, Glyph};
use crate::geometry::{Dimensions, Point};
use crate::graphics::{SpriteMap};
use glium::{Frame, Program, Surface, Blend};
use glium::backend::glutin::Display;
use glium::uniforms::Sampler;
use glium::uniforms::MagnifySamplerFilter::Nearest;

/// The root object representing what is drawn to the screen.
pub struct Terminal {
    pub dims: Dimensions,

    pub root_panel: Panel,
}

impl Terminal {
    /// Creates a new Terminal with the given Dimensions.
    pub fn new(dims: Dimensions) -> OozeResult<Terminal> {
        let terminal = Terminal {
            dims,
            root_panel: Panel::new(dims)?,
        };

        Ok(terminal)
    }

    /// Collects the glyphs from alll this terminal's sub-panels and draws them to the screen ordered by layer.
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

    /// Collects a Vector of (Glyph, final_point, layer) from each sub-panel.
    fn collect_drawable_glyphs(&self) -> Vec<(&Glyph, Point, usize)> {
        let mut result: Vec<(&Glyph, Point, usize)> = Vec::new();
        let panels: Vec<&Panel> = self.root_panel.all_sub_panels();
        for panel in panels {
            if !panel.hidden {
                for glyph in panel.glyphs() {
                    result.push((&glyph, glyph.location.plus(panel.dims.offset), panel.layer));
                }
            }
        };
        result.sort_by_key(|(_, _, layer)| *layer);
        result
    }
}



