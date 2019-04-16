use std::path::Path;

use ooze;
use ooze::app::{App, GameState, OozeResult};
use ooze::geometry::{Dimensions, Point};

fn main() -> ooze::app::OozeResult<()> {
    // Dimension for the screen as Sprite size, terminal size, offset
    let screen_dims = Dimensions::new(8, 8, 80, 50, 0, 0);
    // Upscale the sprites from 8x8 to 16x16
    let scale = 2.0;

    // Create the main application object
    let mut a = App::new(screen_dims, scale, "Ooze", Path::new(r#"resources\sheets\test_sheet.png"#))?;

    // Dimensions for the sub-panel copy from the screen with some changes
    let panel_dims = screen_dims.copy_for_panel(Point::new(60, 30), Point::new(10, 10));

    // Add a new sub-panel to the root panel and fill it with randomly colored "@"'s
    a.terminal.root_panel.add_sub_panel_with(panel_dims)?;
    a.terminal.root_panel.sub_panels[0].fill_with_random()?;

    // Dimensions for the sub-panel of the sub-panel copy from the screen as well
    let panel_dims = screen_dims.copy_for_panel(Point::new(50, 20), Point::new(5, 5));

    // Add a panel with these dimensions to the sub-panel above
    a.terminal.root_panel.sub_panels[0].add_sub_panel_with(panel_dims)?;

    // Set the application's `update` callback to the one defined below, it is called once each frame.
    a.update_callback = test_update;

    // Create an empty gamestate
    let mut g = MyGameState{};

    // Start the application loop.
    a.run(&mut g)?;

    Ok(())
}

struct MyGameState {}
impl GameState for MyGameState {
    fn update(&mut self) {}
}

// re-fills the sub-sub-panel with random "@"'s.
fn test_update(app: &mut App<MyGameState>, _game_state: &mut MyGameState) -> OozeResult<()> {
    app.terminal.root_panel.sub_panels[0].sub_panels[0].fill_with_random()?;
    Ok(())
}
