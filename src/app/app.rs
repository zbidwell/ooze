use std::path::Path;

use glium;
use glium::glutin;
use glium::Surface;

use crate::app::OozeResult;
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

    pub update_callback: fn(&mut App<G>, &mut G) -> OozeResult<()>,
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

        let terminal = Terminal::new(dims)?;
        let program = glium::Program::from_source(
            &display,
            get_shader(Path::new(r#"resources\shaders\vertex\v_shader_default.vert"#))?.as_str(),
            get_shader(Path::new(r#"resources\shaders\fragment\f_shader_default.frag"#))?.as_str(),
            None
        )?;
        let sprites = SpriteMap::from_sheet(&display, sprite_sheet_path)?;

        let app = App {
            events_loop,
            display,
            program,
            terminal,
            sprites,
            update_callback: default_update_callback,
        };

        Ok(app)
    }

    /// Calls the given update callback set by the user, which should modify this App's terminal using information from the GameState.
    fn update(&mut self, game_state: &mut G) -> OozeResult<()> {
        (self.update_callback)(self, game_state)?;
        Ok(())
    }

    /// Draw this App's Terminal to the window.
    fn draw(&self) -> OozeResult<()> {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.terminal.draw(&mut target, &self.display, &self.program, &self.sprites)?;

        target.finish()?;

        Ok(())
    }

    /// Start this App's main loop. Draws the App, handles window events, and calls update on the GameState.
    pub fn run(&mut self, game_state: &mut G) -> OozeResult<()> {
        let mut closed = false;
        while !closed {
            // clear, draw the terminal, and flip the window
            self.draw()?;

            // Handle all window events
            self.events_loop.poll_events(|ev| match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            });

            self.update(game_state)?;
        }

        Ok(())
    }
}

/// Is the default upon App creation, does nothing.
fn default_update_callback<G: GameState>(_app: &mut App<G>, _game_state: &mut G) -> OozeResult<()> {
    Ok(())
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
    let display =
        glium::Display::new(window, context, &events_loop)?;

    Ok((events_loop, display))
}
