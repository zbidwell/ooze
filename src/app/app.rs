use std::time::Instant;
use std::collections::HashMap;

use glium;
use glium::glutin;
use glium::Surface;

use crate::graphics::{Drawable, Dimensions, F_SHADER, V_SHADER, Sprite, SpriteId};
use crate::graphics::Terminal;

pub struct App {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub program: glium::Program,

    pub terminal: Terminal,

    pub sprites: HashMap<SpriteId, Sprite>,
}

impl App {
    pub fn new(dims: Dimensions, title: &str) -> App {
        let (events_loop, display) =
            init_window(
                dims.glyph_width * dims.term_width,
                dims.glyph_height * dims.term_height,
                title
            );

        let terminal = Terminal::new(dims);
        let program = glium::Program::from_source(&display, V_SHADER, F_SHADER, None).unwrap();
        let mut sprites = HashMap::new();
        sprites.insert(SpriteId{id:"empty"}, Sprite::new(r#"C:\RustProjects\Ooze\src\res\empty.png"#, &display));
        sprites.insert(SpriteId{id:"a"}, Sprite::new(r#"C:\RustProjects\Ooze\src\res\a.png"#, &display));

        App {
            events_loop,
            display,
            program,
            terminal,
            sprites,
        }
    }

    fn update(&mut self) {
        self.terminal.root_pane.fill_with_random();
    }

    fn draw(&self) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.terminal.draw(&mut target, &self.display, &self.program, &self.sprites);

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

            self.update();

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
