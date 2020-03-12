extern crate glutin;

#[derive(Debug, PartialEq)]
pub enum Event {
    Closed,
}

pub fn convert(event: glutin::Event) -> Option<Event> {
    match event {
        glutin::Event::WindowEvent{ event, .. } => 
            match event {
                glutin::WindowEvent::Closed => Option::Some(Event::Closed),
                _ => Option::None
            }
        _ => Option::None
    }
}