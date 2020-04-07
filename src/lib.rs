use balaklava_gpu::Device;
use balaklava_gfx::config::Config;
use balaklava_gfx::GfxDevice;
use balaklava_gfx::glutin::{EventsLoop, Event, WindowEvent};

pub trait Application<D: Device> {
    fn new(device: &mut D) -> Self;
    fn render(&mut self, device: &mut D);
}


pub fn lauch_gfx<A: Application<GfxDevice>>(config: Config) {
    let mut events_loop = EventsLoop::new();
    let mut device = GfxDevice::new(config, &events_loop);
    let mut app = A::new(&mut device);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Closed => running = false,
                _ => ()
            },
            _ => ()
        });

        app.render(&mut device);
        device.flush();
    }
}