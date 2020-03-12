extern crate glutin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum KeyCodes {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

#[derive(Debug, PartialEq)]
pub struct KeyInput {
    scancode: u32,
    virtual_key: Option<KeyCodes>
}

impl std::fmt::Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyInput(scancode: {}, {:?})", self.scancode, self.virtual_key)
    } 
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Closed,
    KeyInput { 
        input: KeyInput 
    }
}

fn map_keycode(virtual_key: Option<glutin::VirtualKeyCode>) -> Option<KeyCodes> {
    if virtual_key.is_none() {
        return Option::None
    }

    match virtual_key.unwrap() {
        glutin::VirtualKeyCode::A => Option::Some(KeyCodes::A), 
        glutin::VirtualKeyCode::B => Option::Some(KeyCodes::B), 
        glutin::VirtualKeyCode::C => Option::Some(KeyCodes::C), 
        glutin::VirtualKeyCode::D => Option::Some(KeyCodes::D), 
        glutin::VirtualKeyCode::E => Option::Some(KeyCodes::E), 
        glutin::VirtualKeyCode::F => Option::Some(KeyCodes::F), 
        glutin::VirtualKeyCode::G => Option::Some(KeyCodes::G), 
        glutin::VirtualKeyCode::H => Option::Some(KeyCodes::H), 
        glutin::VirtualKeyCode::I => Option::Some(KeyCodes::I), 
        glutin::VirtualKeyCode::J => Option::Some(KeyCodes::J), 
        glutin::VirtualKeyCode::K => Option::Some(KeyCodes::K), 
        glutin::VirtualKeyCode::L => Option::Some(KeyCodes::L), 
        glutin::VirtualKeyCode::M => Option::Some(KeyCodes::M), 
        glutin::VirtualKeyCode::N => Option::Some(KeyCodes::N), 
        glutin::VirtualKeyCode::O => Option::Some(KeyCodes::O), 
        glutin::VirtualKeyCode::P => Option::Some(KeyCodes::P), 
        glutin::VirtualKeyCode::Q => Option::Some(KeyCodes::Q), 
        glutin::VirtualKeyCode::R => Option::Some(KeyCodes::R), 
        glutin::VirtualKeyCode::S => Option::Some(KeyCodes::S), 
        glutin::VirtualKeyCode::T => Option::Some(KeyCodes::T), 
        glutin::VirtualKeyCode::U => Option::Some(KeyCodes::U), 
        glutin::VirtualKeyCode::V => Option::Some(KeyCodes::V), 
        glutin::VirtualKeyCode::W => Option::Some(KeyCodes::W), 
        glutin::VirtualKeyCode::X => Option::Some(KeyCodes::X), 
        glutin::VirtualKeyCode::Y => Option::Some(KeyCodes::Y), 
        glutin::VirtualKeyCode::Z => Option::Some(KeyCodes::Z),
        _ => Option::None
    }
}

pub fn convert(event: glutin::Event) -> Option<Event> {
    match event {
        glutin::Event::WindowEvent{ event, .. } => 
            match event {
                glutin::WindowEvent::Closed => Option::Some(Event::Closed),
                glutin::WindowEvent::KeyboardInput { input, .. } => Option::Some(Event::KeyInput {
                    input: KeyInput { 
                        scancode: input.scancode,
                        virtual_key: map_keycode(input.virtual_keycode)
                    }
                }),
                _ => Option::None
            }
        _ => Option::None
    }
}