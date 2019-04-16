use crate::terminal::{Glyph};
use crate::geometry::{Dimensions, Point, Rect};
use crate::app::{OozeResult, OozeError};

use rand;

pub struct Pane {
    pub dims: Dimensions,

    pub layer: usize,
    pub hidden: bool,

    pub contents: Vec<Vec<Glyph>>,

    pub sub_panes: Vec<Pane>,
}

impl Pane {
    pub fn new(dims: Dimensions) -> OozeResult<Pane> {
        let pane = Pane {
            dims,
            layer: 0,
            hidden: false,
            // 2D [x][y] Vec with capacity of [width][height]
            contents: {
                let mut outer = Vec::with_capacity(dims.term_size.x as usize);
                for x in 0..dims.term_size.x {
                    outer.push(Vec::with_capacity(dims.term_size.y as usize));
                    for y in 0..dims.term_size.y {
                        outer[x as usize].push(Glyph::new(
                            Point::new(x, y),
                            [1.0, 1.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 0.0],
                            "empty".to_string(),
                        )?);
                    }
                }
                outer
            },
            sub_panes: Vec::new(),
        };

        Ok(pane)
    }

    pub fn hide(&mut self) {
        if !self.hidden {
            self.hidden = true;
        }
    }

    pub fn show(&mut self) {
        if self.hidden {
            self.hidden = false;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::of_size(self.dims.term_size)
    }

    pub fn add_sub_pane(&mut self, mut pane: Pane) -> OozeResult<()> {
        if !self.rect().contains_rect(pane.dims.rect()) {
            return Err(Box::new(OozeError))
        }
        
        pane.dims.offset = pane.dims.offset.plus(self.dims.offset);
        pane.layer = self.layer + 1;

        self.sub_panes.push(pane);

        Ok(())
    }

    pub fn set(&mut self, point: Point, glyph: Glyph) -> OozeResult<()> {
        if !self.rect().contains_point(point) {
            return Err(Box::new(OozeError))
        }
        self.contents[point.x as usize][point.y as usize] = glyph;
        Ok(())
    }

    pub fn get(&self, point: Point) -> OozeResult<&Glyph> {
        if !self.rect().contains_point(point) {
            return Err(Box::new(OozeError))
        }
        Ok(&self.contents[point.x as usize][point.y as usize])
    }

    pub fn add_sub_pane_with(&mut self, dims: Dimensions) -> OozeResult<()> {
        let pane = Pane::new(dims)?;
        self.add_sub_pane(pane)?;

        Ok(())
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }

    pub fn place(
        &mut self,
        x: u32,
        y: u32,
        id: &str,
        fg_color: [f32; 4],
        bg_color: [f32; 4]
        ) -> OozeResult<()> {

        let point = Point::new(x, y);
        let glyph = Glyph::new(
            point,
            fg_color,
            bg_color,
            id.to_string(),
        )?;

        self.set(point, glyph)?;

        Ok(())
    }

    pub fn make_border(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) -> OozeResult<()> {
        for point in self.rect().points() {
            if point.x == 0 || point.x == self.rect().size.x - 1 || point.y == 0 || point.y == self.rect().size.y - 1 {
                self.place(point.x, point.y, id, fg_color, bg_color)?;
            }
        }

        Ok(())
    }

    pub fn fill_with(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) -> OozeResult<()> {
        for point in self.rect().points() {
            self.place(point.x, point.y, id, fg_color, bg_color)?;
        }

        Ok(())
    }

    pub fn fill_with_random(&mut self) -> OozeResult<()> {
        for point in self.rect().points() {
            let fg_color = [rand::random(), rand::random(), rand::random(), 1.0];
            let bg_color = [rand::random(), rand::random(), rand::random(), 1.0];
            self.place(point.x, point.y, "@", fg_color, bg_color)?;
        }

        Ok(())
    }

    pub fn glyphs(&self) -> Vec<&Glyph> {
        let mut result = Vec::with_capacity((self.dims.term_size.x * self.dims.term_size.y) as usize);
        for column in &self.contents {
            for glyph in column {
                result.push(glyph);
            }
        }
        result
    }

    pub fn all_sub_panes(&self) -> Vec<&Pane> {
        let mut result: Vec<&Pane> = Vec::new();
        result.push(&self);
        for pane in &self.sub_panes {
            result.extend(pane.all_sub_panes().iter());
        };

        result
    }
}
