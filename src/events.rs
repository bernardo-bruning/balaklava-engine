extern crate glutin;

#[derive(Debug, PartialEq)]
pub struct KeyInput {
    scancode: u32
}

impl std::fmt::Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scancode: {}", self.scancode)
    } 
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Closed,
    KeyInput { input: KeyInput }
}

pub fn convert(event: glutin::Event) -> Option<Event> {
    match event {
        glutin::Event::WindowEvent{ event, .. } => 
            match event {
                glutin::WindowEvent::Closed => Option::Some(Event::Closed),
                glutin::WindowEvent::KeyboardInput { input, .. } => Option::Some(Event::KeyInput {
                    input: KeyInput { scancode: input.scancode }
                }),
                _ => Option::None
            }
        _ => Option::None
    }
}