use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal};

fn main() {
    println!("Hello, world!");

    let mut a = App::new(16, 16, 32, 32, "Ooze");

    a.terminal.root_pane.fill_with_random();

    a.run();
}
