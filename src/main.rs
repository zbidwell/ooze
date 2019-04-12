use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal, Dimensions};

fn main() {
    println!("Hello, world!");

    let mut a = App::new(Dimensions::new(12, 16, 80, 50), "Ooze");

    a.terminal.root_pane.fill_with_random();

    a.run();
}
