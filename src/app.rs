use std::path::Path;

use std::time::Instant;

use glium;
use glium::glutin;
use glium::Surface;

use crate::error::{OozeResult};
use crate::geometry::{Dimensions};
use crate::graphics::{SpriteMap, get_shader};
use crate::terminal::{Terminal};

/// This should be implemented by the user's main Game or GameState struct.
pub trait GameState {
    /// update this object's state, called by the application loop once per frame.
    fn update(&mut self) {}
}

/// The main struct of the Ooze system. Handles windowing, rendering, and contains a GameState defined by the user.
pub struct App<G: GameState> {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub program: glium::Program,

    pub terminal: Terminal,

    pub sprites: SpriteMap,

    pub closed: bool,

    pub update_game_callback: fn(&mut App<G>, &mut G),
    pub handle_events_callback: fn(&mut App<G>, &mut G),
}

impl<G: GameState> App<G> {
    /// Create a new App.
    pub fn new(dims: Dimensions, scale: f32, title: &str, sprite_sheet_path: &Path) -> OozeResult<App<G>> {
        let (events_loop, display) =
            init_window(
                (dims.glyph_size.x as f32 * dims.term_size.x as f32 * scale) as usize,
                (dims.glyph_size.y as f32 * dims.term_size.y as f32 * scale) as usize,
                title
            )?;

        let terminal = Terminal::new(dims);

        let program = glium::Program::from_source(
            &display,
            get_shader(Path::new(r#"resources\shaders\vertex\v_shader_default.vert"#))?.as_str(),
            get_shader(Path::new(r#"resources\shaders\fragment\f_shader_default.frag"#))?.as_str(),
            None
        ).expect("Failed while creating shader program");

        let sprites = SpriteMap::from_sheet(&display, sprite_sheet_path)?;

        let app = App {
            events_loop,
            display,
            program,
            terminal,
            sprites,
            closed: false,
            update_game_callback: default_update_callback,
            handle_events_callback: default_handle_events_callback,
        };

        Ok(app)
    }

    /// Calls the given update callback set by the user, which should modify this App's terminal using information from the GameState.
    fn update_game(&mut self, game_state: &mut G) {
        (self.update_game_callback)(self, game_state);
    }

    fn handle_events(&mut self, game_state: &mut G) {
        (self.handle_events_callback)(self, game_state);
    }

    /// Draw this App's Terminal to the window.
    fn draw(&self) -> OozeResult<()> {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.terminal.draw(&mut target, &self.display, &self.program, &self.sprites)?;

        target.finish().unwrap();

        Ok(())
    }

    /// Start this App's main loop. Draws the App, handles window events, and calls update on the GameState.
    pub fn run(&mut self, game_state: &mut G) -> OozeResult<()> {
        while !self.closed {
            // clear, draw the terminal, and flip the window
            let start = Instant::now();
            self.draw()?;
            println!("{:?}", start.elapsed());

            // Handle all window events
            self.handle_events(game_state);

            // Tell the game_state to update itself
            self.update_game(game_state);
        }

        Ok(())
    }
}

/// Is the default upon App creation, does nothing.
fn default_update_callback<G: GameState>(_app: &mut App<G>, _game_state: &mut G) {}

// Is the default upon App creation, instructs the App to close when the window gets a close event.
fn default_handle_events_callback<G: GameState>(app: &mut App<G>, _game_state: &mut G) {
    let mut events = Vec::new();
    app.events_loop.poll_events(|ev| events.push(ev));

    for event in events {
        if let glutin::Event::WindowEvent { event: window_event, ..} = event {
            if let glutin::WindowEvent::CloseRequested = window_event {
                app.closed = true;
            }
        }

        // match event {
        //     glutin::Event::WindowEvent { event, .. } => match event {
        //     glutin::WindowEvent::CloseRequested => app.closed = true,
        //     _ => (),
        // },
        // _ => (),
        // }
    }
}

/// Creates and returns an event loop and a display, which manages window and OpenGL context
fn init_window(width: usize, height: usize, title: &str) -> OozeResult<(glutin::EventsLoop, glium::Display)> {
    let size = glutin::dpi::LogicalSize::new(width as f64, height as f64);

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(size)
        .with_resizable(false)
        .with_title(title);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop)?;

    Ok((events_loop, display))
}
