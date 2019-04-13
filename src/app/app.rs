use std::time::Instant;
use std::collections::HashMap;

use glium;
use glium::glutin;
use glium::Surface;

use crate::graphics::{Terminal, Dimensions, F_SHADER, V_SHADER, Sprite, SpriteId};

pub struct App {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub program: glium::Program,

    pub terminal: Terminal,

    pub sprites: HashMap<SpriteId, Sprite>,

    pub update_callback: fn(&mut App) -> (),
}

impl App {
    pub fn new(dims: Dimensions, title: &str) -> App {
        let (events_loop, display) =
            init_window(
                (dims.glyph_size.x * dims.term_size.x) as usize,
                (dims.glyph_size.y * dims.term_size.y) as usize,
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
            update_callback: default_update_callback,
        }
    }


    fn update(&mut self) {
        (self.update_callback)(self);
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

fn default_update_callback(app: &mut App) {

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
