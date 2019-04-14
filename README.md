# Ooze

A psuedo-console for roguelike development

![Example Gif](https://github.com/zbidwell/Ooze/resources/example.gif)

```rust
use std::time::Duration;

use ooze;
use ooze::app::App;
use ooze::graphics::{Dimensions, Point};

fn main() {
    // Dimension for the screen as Sprite size, terminal size, offset
    let screen_dims = Dimensions::new(
        Point::new(8, 8),
        Point::new(80, 50),
        Point::new(0, 0)
    );
    // Upscale the sprites from 8x8 to 16x16
    let scale = 2.0;

    // Create the main application object
    let mut a = App::new(screen_dims, scale, "Ooze");

    // Dimensions for the sub-pane copy from the screen with some changes
    let pane_dims = screen_dims
        .with_term_size(Point::new(60, 30))
        .with_offset(Point::new(10, 10));

    // Add a new sub-pane to the root pane and fill it with randomly colored "@"'s
    a.terminal.root_pane.add_sub_pane_with(pane_dims);
    a.terminal.root_pane.sub_panes[0].fill_with_random();

    // Dimensions for the sub-pane of the sub-pane copy from the screen as well
    let pane_dims = screen_dims
        .with_term_size(Point::new(50, 20))
        .with_offset(Point::new(5, 5));

    // Add a pane with these dimensions to the sub-pane above
    a.terminal.root_pane.sub_panes[0].add_sub_pane_with(pane_dims);

    // Set the application's `update` callback to the one defined below, it is called once each frame.
    a.update_callback = test_update;

    // Start the application loop.
    a.run();
}

// re-fills the sub-sub-pane with random "@"'s.
fn test_update(app: &mut App, dt: Duration) {
    app.terminal.root_pane.sub_panes[0].sub_panes[0].fill_with_random();
}
```