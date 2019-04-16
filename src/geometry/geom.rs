use crate::graphics::Vertex;

/// A 2D point with x and y both positive integers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    /// Create a new Point. Origin is at bottom-left.
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    /// Calculate the vertices for a quad on the screen, returns as [top-left, top-right, bottom-left, bottom-right] where self is at bottom-left.
    pub fn screen_verts(&self, terminal_dims: Dimensions) -> [Vertex; 4] {
        [
            Vertex::from_arrays(self.plus(Point::new(0, 1)).to_screen(terminal_dims), [0.0, 1.0]),
            Vertex::from_arrays(self.plus(Point::new(1, 1)).to_screen(terminal_dims), [1.0, 1.0]),
            Vertex::from_arrays(self.to_screen(terminal_dims), [0.0, 0.0]),
            Vertex::from_arrays(self.plus(Point::new(1, 0)).to_screen(terminal_dims), [1.0, 0.0]),
        ]
    }

    /// Converts "terminal" coordinates to OpenGL screen coordinates. (i.e. from [0, terminal_size - 1] integer space to [-1, 1] float space)
    pub fn to_screen(&self, terminal_dims: Dimensions) -> [f32; 2] {
        [
            2.0 * ((self.x as f32) / terminal_dims.term_size.x as f32) - 1.0,
            2.0 * ((self.y as f32) / terminal_dims.term_size.y as f32) - 1.0
        ]
    }

    /// Adds points like vectors and returns a new point
    pub fn plus(&self, other: Point) -> Point {
        // TODO: overload add?
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// A 2D rectangle in positive integer coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub bottom_left: Point,
    pub size: Point,
}

impl Rect {
    /// Create a new rectangle. Size is a Point(width, height).
    pub fn new(bottom_left: Point, size: Point) -> Rect {
        Rect { bottom_left, size }
    }

    /// Create a rect with the bottom-left at the origin.
    pub fn of_size(size: Point) -> Rect {
        Rect::new(Point::new(0, 0), size)
    }

    /// Return a Vector of all the Points contained in this Rect.
    pub fn points(&self) -> Vec<Point> {
        let mut result = Vec::with_capacity((self.size.x * self.size.y) as usize);
        for x in self.bottom_left.x..self.bottom_left.x+self.size.x {
            for y in self.bottom_left.y..self.bottom_left.y+self.size.y {
                result.push(Point::new(x, y));
            }
        }
        result
    }

    /// Check if the given Point is contained within this Rect.
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.bottom_left.x &&
        point.x < self.bottom_left.x + self.size.x &&
        point.y >= self.bottom_left.y &&
        point.y < self.bottom_left.y + self.size.y
    }

    /// Check if a given Rect is fully contained inside this Rect.
    pub fn contains_rect(&self, rect: Rect) -> bool {
        let bl = rect.bottom_left;
        let tr = rect.bottom_left.plus(Point::new(rect.size.x - 1, rect.size.y - 1));
        self.contains_point(bl) && self.contains_point(tr)
    }
}

/// Dimensions for the creation of window-like objects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub glyph_size: Point,
    pub term_size: Point,
    pub offset: Point,
}

impl Dimensions {
    /// Create a new Dimensions from simple values.
    pub fn new(glyph_width: u32, glyph_height: u32, term_width: u32, term_height: u32, offset_x: u32, offset_y: u32) -> Dimensions {
        Dimensions {
            glyph_size: Point::new(glyph_width, glyph_height),
            term_size: Point::new(term_width, term_height),
            offset: Point::new(offset_x, offset_y),
        }
    }

    /// Create a new Dimensions from Points.
    pub fn from_sizes(glyph_size: Point, term_size: Point, offset: Point) -> Dimensions {
        Dimensions {
            glyph_size,
            term_size,
            offset,
        }
    }

    /// Return the Rect that these dimensions would cover on a parent's Rect.
    pub fn rect(&self) -> Rect {
        Rect::new(self.offset, self.term_size)
    }

    /// Make a copy of these Dimensions with the same glyph_size, but new term_size and offset.
    pub fn copy_for_panel(&self, term_size: Point, offset: Point) -> Dimensions {
        self.clone().with_term_size(term_size).with_offset(offset)
    }

    /// Return a new Dimensions with a changed glyph_size.
    pub fn with_glyph_size(&self, glyph_size: Point) -> Dimensions {
        let mut new = self.clone();
        new.glyph_size = glyph_size;
        new
    }

    /// Return a new Dimensions with a changed term_size.
    pub fn with_term_size(&self, term_size: Point) -> Dimensions {
        let mut new = self.clone();
        new.term_size = term_size;
        new
    }

    /// Return a new Dimensions with a changed offset.
    pub fn with_offset(&self, offset: Point) -> Dimensions {
        let mut new = self.clone();
        new.offset = offset;
        new
    }
}