#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::{GlContext};

gfx_defines!{
    vertex Vertex {
        position: [f32; 4] = "vertex_position",
        normal: [f32; 3] = "vertex_normal",
        color: [f32; 3] = "vertex_color",
    }

    constant Light {
        position: [f32; 4] = "light_position",
        color: [f32; 3] = "light_color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        light: gfx::ConstantBuffer<Light> = "light",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "target",
    }
}

struct Engine<'a> {
    pub meshes: &'a[&'a[Vertex]],
    pub lights: &'a[Light]
}

impl <'a> Engine<'a> {
    fn new(meshes: &'a[&'a[Vertex]], lights: &'a[Light]) 
        -> Result<Engine<'a>, String>{
        return Ok(Engine {
            meshes,
            lights
        })
    }

    fn run(self) {
        let mut event_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_dimensions(500, 400)
            .with_title("Teste Aplicativo");
            
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
            .with_vsync(true);
            
        let (window, mut device, mut factory, color, _depth_view) =
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

        let light_buffer = factory.create_constant_buffer(1);

        let mesh = self.meshes[0];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(mesh, ());
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
            encoder.update_buffer(&data.light, &self.lights, 0).unwrap();
            encoder.clear(&color, [0.0, 0.0, 0.0, 1.0]);
            encoder.draw(&slice, &pso, &data);
            encoder.flush(&mut device);
        }
    }
}

fn main() {
    let meshes = &[&[
        Vertex { 
            position: [ -0.5, -0.5, 0.0, 1.0 ], 
            normal: [0.0, 0.0, 1.0], 
            color: [1.0, 0.0, 0.0] 
        },
        Vertex { 
            position: [  0.5, -0.5, 0.0, 1.0 ], 
            normal: [0.0, 0.0, 1.0], 
            color: [0.0, 1.0, 0.0] 
        },
        Vertex { 
            position: [  0.0,  0.5, 0.0, 1.0 ], 
            normal: [0.0, 0.0, 1.0], 
            color: [0.0, 0.0, 1.0] 
        },
    ][0..3]];

    let lights = &[
        Light {
            position: [0.0, 0.0, 0.07, 1.0],
            color: [1.0, 1.0, 1.0]
        }
    ][0..1];

    let engine = Engine::new(meshes, lights).unwrap();
    engine.run();
}