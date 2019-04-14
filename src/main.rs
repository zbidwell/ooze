use std::time::Duration;

use ooze;
use ooze::app::App;
use ooze::graphics::{Glyph, Pane, Terminal, Dimensions, Point};

fn main() {
    let screen_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(80, 50),
        Point::new(0, 0)
    );
    let scale = 2.0;

    let mut a = App::new(screen_dims, scale, "Ooze");

    let pane_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(60, 30),
        Point::new(10, 10)
    );
    a.terminal.root_pane.add_sub_pane_with(pane_dims);
    a.terminal.root_pane.sub_panes[0].fill_with_random();

    let pane_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(50, 20),
        Point::new(5, 5)
    );
    a.terminal.root_pane.sub_panes[0].add_sub_pane_with(pane_dims);

    a.update_callback = test_update;

    a.run();
}

fn test_update(app: &mut App, dt: Duration) {
    app.terminal.root_pane.sub_panes[0].sub_panes[0].fill_with_random();
}
