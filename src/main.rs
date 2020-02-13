#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
use gfx::Device;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform{
        transform: [[f32;4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "Target0",
    }
}

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