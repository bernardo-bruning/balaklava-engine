extern crate glutin;

#[derive(Debug, PartialEq)]
pub enum KeyCodes {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

#[derive(Debug, PartialEq)]
pub struct KeyInput {
    scancode: u32,
    //virtual_key: KeyCodes
}

impl std::fmt::Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scancode: {}", self.scancode)
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

    match virtual_key {
        A => Option::Some(KeyCodes::A), 
        B => Option::Some(KeyCodes::B), 
        C => Option::Some(KeyCodes::C), 
        D => Option::Some(KeyCodes::D), 
        E => Option::Some(KeyCodes::E), 
        F => Option::Some(KeyCodes::F), 
        G => Option::Some(KeyCodes::G), 
        H => Option::Some(KeyCodes::H), 
        I => Option::Some(KeyCodes::I), 
        J => Option::Some(KeyCodes::J), 
        K => Option::Some(KeyCodes::K), 
        L => Option::Some(KeyCodes::L), 
        M => Option::Some(KeyCodes::M), 
        N => Option::Some(KeyCodes::N), 
        O => Option::Some(KeyCodes::O), 
        P => Option::Some(KeyCodes::P), 
        Q => Option::Some(KeyCodes::Q), 
        R => Option::Some(KeyCodes::R), 
        S => Option::Some(KeyCodes::S), 
        T => Option::Some(KeyCodes::T), 
        U => Option::Some(KeyCodes::U), 
        V => Option::Some(KeyCodes::V), 
        W => Option::Some(KeyCodes::W), 
        X => Option::Some(KeyCodes::X), 
        Y => Option::Some(KeyCodes::Y), 
        Z => Option::Some(KeyCodes::Z),
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
                        scancode: input.scancode

                    }
                }),
                _ => Option::None
            }
        _ => Option::None
    }
}