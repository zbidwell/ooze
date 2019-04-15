use ooze;
use ooze::app::*;
use ooze::graphics::*;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::thread;
use std::time::Duration;

fn main() {
    // App initialize
    let mut app = App::new(Dimensions::new(16, 16, 10, 10, 0, 0), 2.0, "Showoff", r#"resources\sheets\showoff.png"#);
    // position for our slime
    let mut game_state = MyGameState{pos:[6, 2]};

    // Use root_pane for walls and floors
    app.terminal.root_pane.fill_with("floor", [0.07, 0.04, 0.06, 1.0], [0.0, 0.0, 0.0, 1.0]);
    app.terminal.root_pane.make_border("wall", [0.09, 0.03, 0.04, 1.0], [0.0, 0.0, 0.0, 1.0]);

    // Add pane above that for the ooze
    app.terminal.root_pane.add_sub_pane_with(app.terminal.root_pane.dims);
    app.terminal.root_pane.sub_panes[0].place(6, 2, "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]);

    // Use our update function
    app.update_callback = update;

    // Start the application loop
    app.run(&mut game_state);
}

// Moves the slime around randomly within the room
fn update(app: &mut App<MyGameState>, game_state: &mut MyGameState) {
    app.terminal.root_pane.sub_panes[0].place(game_state.pos[0] as usize, game_state.pos[1] as usize, "empty", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]);
    game_state.update();
    app.terminal.root_pane.sub_panes[0].place(game_state.pos[0] as usize, game_state.pos[1] as usize, "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]);

    thread::sleep(Duration::from_millis(150));
}

// Holds the position of the ooze
struct MyGameState {
    pos: [i32; 2]
}

impl GameState for MyGameState {
    // is called by the app's update callback. Moves the ooze randomly.
    fn update(&mut self) {
        let dirs = [-1, 0, 1];
        let mut x_dir = *dirs.iter().choose(&mut thread_rng()).unwrap();
        let mut y_dir = *dirs.iter().choose(&mut thread_rng()).unwrap();

        if self.pos[0] + x_dir >= 8 || self.pos[0] + x_dir <= 1 {
            x_dir = 0;
        }
        if self.pos[1] + y_dir >= 8 || self.pos[1] + y_dir <= 1 {
            y_dir = 0;
        }

        let new_pos = [x_dir + self.pos[0], y_dir + self.pos[1]];

        self.pos = new_pos;
    }
}