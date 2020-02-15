#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::{GlContext};

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "vertex_position",
        norm: [f32; 3] = "a_Norm",
        color: [f32; 3] = "a_Color",
    }

    constant Light {
        pos: [f32; 4] = "u_Pos",
        color: [f32; 3] = "u_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        light: gfx::ConstantBuffer<Light> = "Light",
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
        
    let (window, mut device, mut factory, color, depth_view) =
        gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
        window_builder, 
        context_builder, 
        &event_loop
    );

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/shader_150.glslv"),
        include_bytes!("shader/shader_150.glslf"),
        pipe::new()
    ).unwrap();

    
    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();
    let triangle: [Vertex; 3] = [
        Vertex { 
            pos: [ -0.5, -0.5, 0.0, 1.0 ], 
            norm: [0.0, 0.0, 1.0], 
            color: [1.0, 0.0, 0.0] 
        },
        Vertex { 
            pos: [  0.5, -0.5, 0.0, 1.0 ], 
            norm: [0.0, 0.0, 1.0], 
            color: [1.0, 0.0, 0.0] 
        },
        Vertex { 
            pos: [  0.0,  0.5, 0.0, 1.0 ], 
            norm: [0.0, 0.0, 1.0], 
            color: [1.0, 0.0, 0.0] 
        },
    ];

    let light = Light {
        pos: [0.0, 0.0, 0.07, 1.0],
        color: [1.0, 1.0, 1.0]
    };

    let light_buffer = factory.create_constant_buffer(1);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&triangle, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        light: light_buffer,
        out: color.clone(),
    };
    
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

        window.swap_buffers().unwrap();
        device.cleanup();
        encoder.update_buffer(&data.light, &[light], 0).unwrap();
        encoder.clear(&color, [0.0, 0.0, 0.0, 1.0]);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
    }
}