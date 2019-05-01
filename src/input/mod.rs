use glium::glutin::{EventsLoop, Event};
use glium::glutin;

pub struct InputHandler {
    events_loop: EventsLoop,
}

impl InputHandler {
    pub fn new(events_loop: EventsLoop) -> InputHandler {
        InputHandler {
            events_loop,
        }
    }

    pub fn poll_events(&mut self) -> Vec<OozeEvent> {
        let mut events = Vec::new();
        self.events_loop.poll_events(|event| events.push(InputHandler::to_ooze_event(event)));
        events
    }

    pub fn to_ooze_event(event: Event) -> OozeEvent {
        match event {
            glutin::Event::WindowEvent { event, ..} => match event {
                glutin::WindowEvent::CloseRequested => {
                    OozeEvent::WindowClose
                },
                glutin::WindowEvent::KeyboardInput { input, ..} => match input {
                    glutin::KeyboardInput {state, virtual_keycode, ..} => {
                        if let glutin::ElementState::Pressed = state {
                            match virtual_keycode {
                                Some(glutin::VirtualKeyCode::Left) => OozeEvent::KeyPress(Key::Left),
                                Some(glutin::VirtualKeyCode::Right) => OozeEvent::KeyPress(Key::Right),
                                Some(glutin::VirtualKeyCode::Up) => OozeEvent::KeyPress(Key::Up),
                                Some(glutin::VirtualKeyCode::Down) => OozeEvent::KeyPress(Key::Down),
                                _ => OozeEvent::Nothing,
                            }
                        } else {
                            OozeEvent::Nothing
                        }
                    }
                    _ => OozeEvent::Nothing
                }
                _ => OozeEvent::Nothing
            },
            _ => OozeEvent::Nothing
        }
    }
}


pub enum OozeEvent {
    WindowClose,
    KeyPress(Key),
    Nothing,
}

pub enum Key {
    Left,
    Right,
    Up,
    Down,
}