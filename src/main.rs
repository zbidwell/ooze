use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal, Dimensions};

fn main() {
    let mut a = App::new(Dimensions::new(8, 8, 80, 50), "Ooze");

    a.run();
}
