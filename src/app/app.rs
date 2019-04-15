use std::path::Path;

use glium;
use glium::glutin;
use glium::Surface;

use crate::graphics::{Dimensions, F_SHADER, V_SHADER, SpriteMap};
use crate::terminal::{Terminal};

pub trait GameState {
    fn update(&mut self) {}
}

pub struct App<G: GameState> {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub program: glium::Program,

    pub terminal: Terminal,

    pub sprites: SpriteMap,

    pub update_callback: fn(&mut App<G>, &mut G) -> (),
}

impl<G: GameState> App<G> {
    pub fn new(dims: Dimensions, scale: f32, title: &str, sprite_sheet: &str) -> App<G> {
        let (events_loop, display) =
            init_window(
                (dims.glyph_size.x as f32 * dims.term_size.x as f32 * scale) as usize,
                (dims.glyph_size.y as f32 * dims.term_size.y as f32 * scale) as usize,
                title
            );

        let terminal = Terminal::new(dims);
        let program = glium::Program::from_source(&display, V_SHADER.as_str(), F_SHADER.as_str(), None).unwrap();
        let sprites = SpriteMap::from_sheet(&display, Path::new(sprite_sheet));

        App {
            events_loop,
            display,
            program,
            terminal,
            sprites,
            update_callback: default_update_callback,
        }
    }


    fn update(&mut self, game_state: &mut G) {
        (self.update_callback)(self, game_state);
    }

    fn draw(&self) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.terminal.draw(&mut target, &self.display, &self.program, &self.sprites);

        target.finish().expect("Failed to flip buffers");
    }

    pub fn run(&mut self, game_state: &mut G) {
        let mut closed = false;
        while !closed {
            // clear, draw the terminal, and flip the window
            self.draw();

            // Handle all window events
            self.events_loop.poll_events(|ev| match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            });

            self.update(game_state);
        }
    }
}

// never called
fn default_update_callback<G: GameState>(_app: &mut App<G>, _game_state: &mut G) {}

// Create an event loop and context
fn init_window(width: usize, height: usize, title: &str) -> (glutin::EventsLoop, glium::Display) {
    let size = glutin::dpi::LogicalSize::new(width as f64, height as f64);

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(size)
        .with_resizable(false)
        .with_title(title);
    let context = glutin::ContextBuilder::new();
    let display =
        glium::Display::new(window, context, &events_loop).expect("Could not create display.");

    (events_loop, display)
}
