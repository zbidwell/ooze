# Ooze

A pseudo-console for roguelike development

![Example Gif](https://github.com/zbidwell/Ooze/blob/master/resources/example.gif)

The code for the above example.
```rust
use ooze;
use ooze::app::*;
use ooze::graphics::*;
use ooze::terminal::*;

use std::time::Duration;
use rand::random;
use std::thread;

fn main() {
    // App initialize
    let mut app = App::new(Dimensions::new(16, 16, 10, 10, 0, 0), 2.0, "Showoff", r#"resources\sheets\showoff.png"#);
    // position for our slime
    app.game_state = [6, 2];

    // Use root_pane for walls and floors
    app.terminal.root_pane.fill_with("floor", [0.07, 0.04, 0.06, 1.0], [0.0, 0.0, 0.0, 1.0]);
    app.terminal.root_pane.make_border("wall", [0.09, 0.03, 0.04, 1.0], [0.0, 0.0, 0.0, 1.0]);

    // Add pane above that for the ooze
    app.terminal.root_pane.add_sub_pane_with(app.terminal.root_pane.dims);
    app.terminal.root_pane.sub_panes[0].place(6, 2, "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]);

    // Use our update function
    app.update_callback = update;

    app.run()
}

// Move the slime around randomly within the room.
fn update(app: &mut App, dt: Duration) {
    // pick a random direction, update some game state, draw the ooze at the new location
}
```