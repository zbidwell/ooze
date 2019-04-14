use crate::graphics::{Point};

pub struct Glyph {
    pub location: Point,

    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub sprite_id: String,
}

impl Glyph {
    pub fn new(location: Point, fg_color: [f32; 4], bg_color: [f32; 4], sprite_id: String) -> Glyph {
        Glyph {
            location,
            fg_color,
            bg_color,
            sprite_id,
        }
    }

    // use by drawing functions to see if this hides lower glyphs
    pub fn is_opaque(&self) -> bool {
        self.bg_color[3] == 1.0
    }
}

