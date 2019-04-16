use crate::geometry::{Point};
use crate::app::OozeError;

/// A glyph (letter, symbol, tile, etc.) on the screen that contains a location in terminal space, foreground and background colors, and an id to access a sprite.
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    pub location: Point,

    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub sprite_id: String,
}

impl Glyph {
    /// Create a new Glyph
    pub fn new(location: Point, fg_color: [f32; 4], bg_color: [f32; 4], sprite_id: String) -> Result<Glyph, OozeError> {
        for c in &fg_color {
            if *c < 0.0 || *c > 1.0 {
                return Err(OozeError)
            }
        }
        for c in &bg_color {
            if *c < 0.0 || *c > 1.0 {
                return Err(OozeError)
            }
        }
        
        Ok(
            Glyph {
                location,
                fg_color,
                bg_color,
                sprite_id,
            }
        )
    }

    /// Check if the alpha value of the background color is 1.0.
    // use by drawing functions to see if this hides lower glyphs
    pub fn is_opaque(&self) -> bool {
        self.bg_color[3] == 1.0
    }
}

