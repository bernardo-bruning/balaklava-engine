extern crate winit;
extern crate gfx_backend_vulkan;
extern crate gfx_hal;

use winit::{Event, WindowEvent, EventsLoop, WindowBuilder, Window, CreationError};
use winit::dpi::LogicalSize;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use gfx_hal::adapter::PhysicalDevice;
use gfx_hal::{ 
    Instance,
    Adapter,
    QueueFamily,
    Graphics,
    Device,
};

struct LocalState {

}

impl Default for LocalState {
    fn default() -> Self {
        LocalState{}
    }
}

struct HalState {
    instance: back::Instance,
    surface: <back::Backend as gfx_hal::Backend>::Surface,
}

impl HalState {
    fn new(window: &Window) -> Self {
        let instance = back::Instance::create("learning gfx", 1);
        let surface = instance.create_surface(window);
        let adapters = instance.enumerate_adapters();
        let adapter: &Adapter<back::Backend> = adapters
            .first()
            .unwrap();

        let queue_family = adapter
            .queue_families
            .first()
            .unwrap();

        let mut gpu = unsafe {
            adapter
            .physical_device
            .open(&[(&queue_family, &[1.0; 1])])
            .unwrap()
        };

        let queue_group = gpu.queues.take::<Graphics>(queue_family.id()).unwrap();
        let command_queue = queue_group.queues.first().unwrap();

        

        println!("size queue: {}", queue_group.queues.len());
        
        HalState{
            instance,
            surface
        }
    }

    fn clear(&mut self) {
        unimplemented!();
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
