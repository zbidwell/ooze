use crate::graphics::Vertex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
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
pub struct Rect {
    pub bottom_left: Point,
    pub size: Point,
}

impl Rect {
    pub fn new(bottom_left: Point, size: Point) -> Rect {
        Rect { bottom_left, size }
    }

    pub fn of_size(size: Point) -> Rect {
        Rect::new(Point::new(0, 0), size)
    }

    pub fn points(&self) -> Vec<Point> {
        let mut result = Vec::with_capacity((self.size.x * self.size.y) as usize);
        for x in self.bottom_left.x..self.bottom_left.x+self.size.x {
            for y in self.bottom_left.y..self.bottom_left.y+self.size.x {
                result.push(Point::new(x, y));
            }
        }
        result
    }

    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.bottom_left.x &&
        point.x < self.bottom_left.x + self.size.x &&
        point.y >= self.bottom_left.y &&
        point.y < self.bottom_left.y + self.size.y
    }

    pub fn contains_rect(&self, rect: Rect) -> bool {
        let bl = rect.bottom_left;
        let tr = rect.bottom_left.plus(Point::new(rect.size.x - 1, rect.size.y - 1));
        self.contains_point(bl) && self.contains_point(tr)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub glyph_size: Point,
    pub term_size: Point,
    pub offset: Point,
}

impl Dimensions {
    pub fn new(glyph_width: u32, glyph_height: u32, term_width: u32, term_height: u32, offset_x: u32, offset_y: u32) -> Dimensions {
        Dimensions {
            glyph_size: Point::new(glyph_width, glyph_height),
            term_size: Point::new(term_width, term_height),
            offset: Point::new(offset_x, offset_y),
        }
    }

    pub fn from_sizes(glyph_size: Point, term_size: Point, offset: Point) -> Dimensions {
        Dimensions {
            glyph_size,
            term_size,
            offset,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.offset, self.term_size)
    }

    pub fn copy_for_pane(&self, term_size: Point, offset: Point) -> Dimensions {
        self.clone().with_term_size(term_size).with_offset(offset)
    }

    pub fn with_glyph_size(&self, glyph_size: Point) -> Dimensions {
        let mut new = self.clone();
        new.glyph_size = glyph_size;
        new
    }

    pub fn with_term_size(&self, term_size: Point) -> Dimensions {
        let mut new = self.clone();
        new.term_size = term_size;
        new
    }

    pub fn with_offset(&self, offset: Point) -> Dimensions {
        let mut new = self.clone();
        new.offset = offset;
        new
    }
}