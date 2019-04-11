use std::time::Instant;

use glium;
use glium::glutin;
use glium::Surface;

use crate::graphics::Drawable;
use crate::graphics::Terminal;

pub struct App {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,

    pub terminal: Terminal,
}

impl App {
    pub fn new(
        glyph_width: usize,
        glyph_height: usize,
        term_width: usize,
        term_height: usize,
        title: &str,
    ) -> App {
        let (events_loop, display) =
            init_window(glyph_width * term_width, glyph_height * term_height, title);

        let terminal = Terminal::new(glyph_width, glyph_height, term_width, term_height);

        App {
            events_loop,
            display,
            terminal,
        }
    }

    fn draw(&self) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.terminal.draw(&mut target, &self.display, &self.terminal);

        target.finish().expect("Failed to flip buffers");
    }

    pub fn run(&mut self) {
        let mut closed = false;
        while !closed {
            let start = Instant::now();
            // Handle all window events
            self.events_loop.poll_events(|ev| match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            });

            // clear and flip the window
            self.draw();
            println!("{:?}", start.elapsed())
        }
    }
}

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
