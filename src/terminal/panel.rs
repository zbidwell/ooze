use crate::terminal::{Glyph};
use crate::geometry::{Dimensions, Point, Rect};
use crate::app::{OozeResult, OozeError};

use rand;

/// A sort of "sub terminal" that contains glyphs for drawing to the screen. Can contain sub-panels.
pub struct Panel {
    pub dims: Dimensions,

    pub layer: usize,
    pub hidden: bool,

    pub contents: Vec<Vec<Glyph>>,

    pub sub_panels: Vec<Panel>,
}

impl Panel {
    /// Create a new Panel with the given dimensions.
    pub fn new(dims: Dimensions) -> OozeResult<Panel> {
        let panel = Panel {
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
            sub_panels: Vec::new(),
        };

        Ok(panel)
    }

    /// Drawing functions should check Panel.hidden before drawing.
    pub fn hide(&mut self) {
        if !self.hidden {
            self.hidden = true;
        }
    }

    /// Drawing functions should check Panel.hidden before drawing.
    pub fn show(&mut self) {
        if self.hidden {
            self.hidden = false;
        }
    }

    /// A Rect the size of this Panel, bottom-left of Rect at (0, 0).
    pub fn rect(&self) -> Rect {
        Rect::of_size(self.dims.term_size)
    }

    /// Add the given Panel as a sub-panel to this one. Updates the given Panel's offset and sets its layer to this Panel's + 1.
    pub fn add_sub_panel(&mut self, mut panel: Panel) -> OozeResult<()> {
        if !self.rect().contains_rect(panel.dims.rect()) {
            return Err(Box::new(OozeError))
        }
        
        panel.dims.offset = panel.dims.offset.plus(self.dims.offset);
        panel.layer = self.layer + 1;

        self.sub_panels.push(panel);

        Ok(())
    }

    /// Set the Glyph at the given Point.
    pub fn set(&mut self, point: Point, glyph: Glyph) -> OozeResult<()> {
        if !self.rect().contains_point(point) {
            return Err(Box::new(OozeError))
        }
        self.contents[point.x as usize][point.y as usize] = glyph;
        Ok(())
    }

    /// get a reference to the Glyph at the given Point.
    pub fn get(&self, point: Point) -> OozeResult<&Glyph> {
        if !self.rect().contains_point(point) {
            return Err(Box::new(OozeError))
        }
        Ok(&self.contents[point.x as usize][point.y as usize])
    }

    /// Create a new sub-panel with the given dimensions and add it to this one.
    pub fn add_sub_panel_with(&mut self, dims: Dimensions) -> OozeResult<()> {
        let panel = Panel::new(dims)?;
        self.add_sub_panel(panel)?;

        Ok(())
    }

    /// Set the drawing layer of this Panel.
    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }

    /// Place a Glyph with the given info.
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

    /// Make a border of Glyphs with the given info on this Panel.
    pub fn make_border(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) -> OozeResult<()> {
        for point in self.rect().points() {
            if point.x == 0 || point.x == self.rect().size.x - 1 || point.y == 0 || point.y == self.rect().size.y - 1 {
                self.place(point.x, point.y, id, fg_color, bg_color)?;
            }
        }

        Ok(())
    }

    /// Fill the Panel with Glyphs with the given info.
    pub fn fill_with(&mut self, id: &str, fg_color: [f32; 4], bg_color: [f32; 4]) -> OozeResult<()> {
        for point in self.rect().points() {
            self.place(point.x, point.y, id, fg_color, bg_color)?;
        }

        Ok(())
    }

    /// Returns a Vector of references to all the glyphs in this Panel. 
    pub fn glyphs(&self) -> Vec<&Glyph> {
        let mut result = Vec::with_capacity((self.dims.term_size.x * self.dims.term_size.y) as usize);
        for column in &self.contents {
            for glyph in column {
                result.push(glyph);
            }
        }
        result
    }

    /// Returns a Vector of references to all the Panels below this one in the Panel tree.
    pub fn all_sub_panels(&self) -> Vec<&Panel> {
        let mut result: Vec<&Panel> = Vec::new();
        result.push(&self);
        for panel in &self.sub_panels {
            result.extend(panel.all_sub_panels().iter());
        };

        result
    }
}
