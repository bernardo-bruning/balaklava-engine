extern crate winit;

use winit::{Event, WindowEvent, EventsLoop, WindowBuilder, Window, CreationError};
use winit::dpi::LogicalSize;

struct WinState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

impl WinState {
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let event_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_dimensions(size)
            .with_title(title)
            .build(&event_loop);
        
        return output.map(|window| Self{
            event_loop,
            window
        });
    }

    pub fn run(mut self) {
        let mut running = true;

        while running {
            self.event_loop.poll_events(|event| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => running = false,
                _ => ()
            });
        }
    }
}

impl Default for WinState {
    fn default() -> Self{
        Self::new("Learn GFX", LogicalSize::new(800.0, 600.0))
        .expect("Could not create a window!")
    }
}

fn main() {
    let win_state = WinState::default();
    win_state.run();
}
