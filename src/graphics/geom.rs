use crate::graphics::Vertex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }

    // vertices as tl, tr, bl, br with self as bottom left
    pub fn screen_verts(&self, terminal_dims: Dimensions) -> [Vertex; 4] {
        [
            Vertex::from_arrays(self.plus(Point::new(0, 1)).to_screen(terminal_dims), [0.0, 1.0]),
            Vertex::from_arrays(self.plus(Point::new(1, 1)).to_screen(terminal_dims), [1.0, 1.0]),
            Vertex::from_arrays(self.to_screen(terminal_dims), [0.0, 0.0]),
            Vertex::from_arrays(self.plus(Point::new(1, 0)).to_screen(terminal_dims), [1.0, 0.0]),
        ]
    }

    pub fn to_screen(&self, terminal_dims: Dimensions) -> [f32; 2] {
        [
            2.0 * ((self.x as f32) / terminal_dims.term_size.x as f32) - 1.0,
            2.0 * ((self.y as f32) / terminal_dims.term_size.y as f32) - 1.0
        ]
    }

    pub fn plus(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub glyph_size: Point,
    pub term_size: Point,
    pub offset: Point,
}

impl Dimensions {
    pub fn new(glyph_size: Point, term_size: Point, offset: Point) -> Dimensions {
        Dimensions {
            glyph_size,
            term_size,
            offset,
        }
    }
}