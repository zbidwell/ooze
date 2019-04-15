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

// Moves the slime around randomly within the room
fn update(app: &mut App, dt: Duration) {
    let pos = app.game_state;
    let mut x_dir = if random::<f32>() > 0.5 {
        1
    } else {
        -1
    };
    let mut y_dir = if random::<f32>() > 0.5 {
        1
    } else {
        -1
    };

    if pos[0] + x_dir >= 8 || pos[0] + x_dir <= 1 {
        x_dir = 0;
    }
    if pos[1] + y_dir >= 8 || pos[1] + y_dir <= 1 {
        y_dir = 0;
    }

    let new_pos = [x_dir + pos[0], y_dir + pos[1]];
    app.game_state = new_pos;

    app.terminal.root_pane.sub_panes[0].place(pos[0] as usize, pos[1] as usize, "empty", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]);
    app.terminal.root_pane.sub_panes[0].place(new_pos[0] as usize, new_pos[1] as usize, "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]);

    thread::sleep_ms(150);
}