#[allow(unused_imports)]
use balaklava_gl::glium::{glutin, Surface};
pub use balaklava_gpu as gpu;
pub use balaklava_math as math;
use balaklava_gpu::Device;
use balaklava_gl::GlDevice;
use balaklava_gfx::config::Config;
use balaklava_gfx::GfxDevice;
use balaklava_gfx::glutin::{EventsLoop, Event, WindowEvent};
pub use math::Vector;

pub mod g2d;

pub trait Application<D: Device> {
    fn new(device: &mut D) -> Self;
    fn render(&mut self, device: &mut D);
}

pub fn lauch_gl<A: Application<GlDevice> + 'static>() {
    let events_loop = 
        glutin::event_loop::EventLoop::new();
    let mut device = GlDevice::new(&events_loop);
    let mut app = A::new(&mut device);
    
    events_loop.run(move |event, _, control_flow| {
        use glutin::event_loop::ControlFlow;
        use glutin::event::{Event, WindowEvent};
        match event {
            Event::WindowEvent{ event, ..} => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            _ => ()
        }
        app.render(&mut device);
        device.flush();       
        std::thread::sleep(std::time::Duration::from_millis(1));
    });
}

#[deprecated(note="method not supported use lauch_gl")]
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