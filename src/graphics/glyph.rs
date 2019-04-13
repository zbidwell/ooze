use crate::graphics::{SpriteId, Point};

pub struct Glyph {
    pub location: Point,

    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub sprite_id: SpriteId,
}

impl Glyph {
    pub fn new(location: Point, fg_color: [f32; 4], bg_color: [f32; 4], sprite_id: SpriteId) -> Glyph {
        Glyph {
            location,
            fg_color,
            bg_color,
            sprite_id,
        }
    }
}

