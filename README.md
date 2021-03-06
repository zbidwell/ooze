# Ooze

A pseudo-console for roguelike development

![Example Gif](https://github.com/zbidwell/Ooze/blob/master/resources/example.gif)

The code for the above example shown and can be run with 
```
cargo run --release --example showoff
```

```rust
use ooze;
use ooze::app::*;
use ooze::geometry::*;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::thread;
use std::time::Duration;

fn main() -> OozeResult<()> {
    // App initialize
    let mut app = App::new(Dimensions::new(16, 16, 10, 10, 0, 0), 2.0, "Showoff", r#"resources\sheets\showoff.png"#)?;
    // position for our slime
    let mut game_state = MyGameState{pos:[6, 2]};

    // Use root_panel for walls and floors
    app.terminal.root_panel.fill_with("floor", [0.07, 0.04, 0.06, 1.0], [0.0, 0.0, 0.0, 1.0])?;
    app.terminal.root_panel.make_border("wall", [0.09, 0.03, 0.04, 1.0], [0.0, 0.0, 0.0, 1.0])?;

    // Add panel above that for the ooze
    app.terminal.root_panel.add_sub_panel_with(app.terminal.root_panel.dims)?;
    app.terminal.root_panel.sub_panels[0].place(6, 2, "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0])?;

    // Use our update function
    app.update_callback = update;

    // Start the application loop
    app.run(&mut game_state)?;

    Ok(())
}

// Moves the slime around randomly within the room
fn update(app: &mut App<MyGameState>, game_state: &mut MyGameState) -> OozeResult<()> {
    app.terminal.root_panel.sub_panels[0].place(game_state.pos[0], game_state.pos[1], "empty", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0])?;
    game_state.update();
    app.terminal.root_panel.sub_panels[0].place(game_state.pos[0], game_state.pos[1], "ooze", [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0])?;

    thread::sleep(Duration::from_millis(150));

    Ok(())
}

// Holds the position of the ooze
struct MyGameState {
    pos: [u32; 2]
}

impl GameState for MyGameState {
    // is called by the app's update callback. Moves the ooze randomly.
    fn update(&mut self) {
        let dirs: [i32; 3] = [-1, 0, 1];
        let mut x_dir = *dirs.iter().choose(&mut thread_rng()).unwrap();
        let mut y_dir = *dirs.iter().choose(&mut thread_rng()).unwrap();

        if self.pos[0] as i32 + x_dir >= 8 || self.pos[0] as i32 + x_dir <= 1 {
            x_dir = 0;
        }
        if self.pos[1] as i32 + y_dir >= 8 || self.pos[1] as i32 + y_dir <= 1 {
            y_dir = 0;
        }

        let new_pos = [(x_dir + self.pos[0] as i32) as u32, (y_dir + self.pos[1] as i32) as u32];

        self.pos = new_pos;
    }
}
```