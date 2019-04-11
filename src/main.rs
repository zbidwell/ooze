use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal};

fn main() {
    println!("Hello, world!");

    let mut a = App::new(16, 16, 32, 32, "Ooze");

    let mut p = Pane::new(6, 6, 32, 32);
    p.fill_with_random();

    a.terminal.root_pane.add_sub_pane(p);

    a.run();
}
