extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
use gfx::Device;


fn main() {
    let mut event_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions(500, 400)
        .with_title("Teste Aplicativo");
        
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
        .with_vsync(true);
    
    let (_window, mut _device, _factory, _color, _depth_view) =
        gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
        window_builder, 
        context_builder, 
        &event_loop
    );


    let mut running = true;
    while running {
        event_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => 
                    match event {
                        glutin::WindowEvent::Closed => running = false,
                        _ => ()
                    }
                _ => ()
            }
        });
    }
}