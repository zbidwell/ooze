use crate::graphics::Drawable;
use crate::graphics::Pane;
use glium::Frame;
use glium::backend::glutin::Display;

#[derive(PartialEq, Debug)]
pub struct Terminal {
    pub glyph_width: usize,
    pub glyph_height: usize,

    pub term_width: usize,
    pub term_height: usize,

    pub screen_width: usize,
    pub screen_height: usize,

    pub root_pane: Pane,
}

impl Terminal {
    pub fn new(
        glyph_width: usize,
        glyph_height: usize,
        term_width: usize,
        term_height: usize,
    ) -> Terminal {
        Terminal {
            glyph_width,
            glyph_height,
            term_width,
            term_height,
            screen_width: term_width * glyph_width,
            screen_height: term_height * glyph_height,
            root_pane: Pane::new(glyph_width, glyph_height, term_width, term_height),
        }
    }

    pub fn term_to_screen(&self, location: [usize; 2]) -> [f32; 2] {
        [
            2.0 * ((location[0] as f32) / self.term_width as f32) - 1.0,
            2.0 * ((location[1] as f32) / self.term_height as f32) - 1.0
        ]
    }
}

impl Drawable for Terminal {
    fn draw(&self, target: &mut Frame, display: &Display, terminal: &Terminal) {
        self.root_pane.draw(target, display, &self);
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::Terminal;
    #[test]
    fn term_to_screen() {
        let t = Terminal::new(3, 2, 5, 4);

        let input: [usize; 2] = [3, 2];
        let expected: [f32; 2] = [0.1, 0.0];

        assert_eq!(t.term_to_screen([0, 0]), [-1.0, -1.0]);
        assert_eq!(t.term_to_screen([5, 0]), [1.0, -1.0]);
        assert_eq!(t.term_to_screen([0, 4]), [-1.0, 1.0]);
        assert_eq!(t.term_to_screen([5, 4]), [1.0, 1.0]);
        assert_eq!(t.term_to_screen(input), expected);
    }
}


