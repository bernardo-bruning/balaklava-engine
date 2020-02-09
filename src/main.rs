extern crate winit;

use winit::{Event, WindowEvent, EventsLoop, WindowBuilder, Window, CreationError};
use winit::dpi::LogicalSize;

struct LocalState {

}

impl Default for LocalState {
    fn default() -> Self {
        LocalState{}
    }
}

struct HalState {
}

impl HalState {
    fn new(window: &Window) -> Self {
        HalState{}
    }
}

impl Default for HalState {
    fn default() -> Self {
        HalState{}
    }
}

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

}

impl Default for WinState {
    fn default() -> Self{
        Self::new("Learn GFX", LogicalSize::new(800.0, 600.0))
        .expect("Could not create a window!")
    }
}

struct UserInput {
    end_request: bool
}

impl UserInput {
    fn poll_events(event_loop: &mut EventsLoop) -> Self {
        let mut output = UserInput{ end_request: false };
        event_loop.poll_events(|event| match event {
            Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => {
                output  .end_request = true
            }
            _ => () 
        });
        return output
    }
}

fn main() {
    let mut win_state = WinState::default();
    let hal_state = HalState::new(&win_state.window);
    let local_state = LocalState::default();

    loop {
        let input = UserInput::poll_events(&mut win_state.event_loop);
        if input.end_request {
            break;
        }
    }
}
