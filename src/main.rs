use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal, Dimensions, Point};

fn main() {
    let screen_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(80, 50),
        Point::new(0, 0),
    );

    let mut a = App::new(screen_dims, "Ooze");

    let pane_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(10, 10),
        Point::new(20, 20),
    );
    a.terminal.root_pane.add_sub_pane_with(pane_dims);
    a.terminal.root_pane.sub_panes[0].fill_with_random();

    a.run();
}
