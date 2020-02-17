#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl as back;

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

struct EngineConfiguration<'a> {
    name: String,
    vertex_shader: &'a[u8],
    pixel_shader: &'a[u8]
}

impl <'a> EngineConfiguration<'a> {
    fn default() -> EngineConfiguration<'a> {
        return EngineConfiguration{
            name: "".to_string(),
            vertex_shader: include_bytes!("shader/shader_150.glslv"),
            pixel_shader: include_bytes!("shader/shader_150.glslf"),
        }
    }

    fn with_name(mut self, name: String) -> EngineConfiguration<'a> {
        self.name = name;
        self
    } 

    fn with_vertex_shader(mut self, shader: &'a[u8]) -> EngineConfiguration<'a> {
        self.vertex_shader = shader;
        self
    }

    fn with_pixel_shader(mut self, shader: &'a[u8]) -> EngineConfiguration<'a> {
        self.pixel_shader = shader;
        self
    }
}

struct Engine<'a> {
    pub meshes: &'a[&'a[Vertex]],
    pub lights: &'a[Light],
    event_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    encoder: gfx::Encoder<back::Resources, back::CommandBuffer>,
    pso: gfx::pso::PipelineState<back::Resources, pipe::Meta>
}

enum Event {
    Closed,
}

impl <'a> Engine<'a> {
    fn new(config: EngineConfiguration<'a>, meshes: &'a[&'a[Vertex]], lights: &'a[Light]) 
        -> Result<Engine<'a>, String>{
        let event_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_dimensions(500, 400)
            .with_title(config.name);
            
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
            .with_vsync(true);
            
        let (window, device, mut factory, color, _depth_view) =
            gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
            window_builder, 
            context_builder, 
            &event_loop
        );

        let pso = factory.create_pipeline_simple(
            config.vertex_shader,
            config.pixel_shader,
            pipe::new()
        ).unwrap();

        let encoder: gfx::Encoder<back::Resources, back::CommandBuffer> = 
            factory.create_command_buffer().into();

        return Ok(Engine {
            meshes,
            lights,
            event_loop,
            window,
            device,
            factory,
            color,
            encoder,
            pso
        })
    }
    
    fn poll_event<F>(&mut self, mut callback: F) where F:FnMut(Event) {
        self.event_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => 
                    match event {
                        glutin::WindowEvent::Closed => callback(Event::Closed),
                        _ => ()
                    }
                _ => ()
            }
        });
    }

    fn run(mut self) {
        let light_buffer = self.factory.create_constant_buffer(1);

        let mesh = self.meshes[0];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(mesh, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            light: light_buffer,
            out: self.color.clone(),
        };
        
        let mut running = true;
        while running {
            self.poll_event(|event| match event {
                Event::Closed => running = false
            });

            self.window.swap_buffers().unwrap();
            self.device.cleanup();
            self.encoder.update_buffer(&data.light, &self.lights, 0).unwrap();
            self.encoder.clear(&self.color, [0.0, 0.0, 0.0, 1.0]);
            self.encoder.draw(&slice, &self.pso, &data);
            self.encoder.flush(&mut self.device);
        }
    }
}

fn main() {
    let config = EngineConfiguration::default()
        .with_name("Learn GFX".to_string());

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

    let engine = Engine::new(config, meshes, lights)
        .unwrap();
    engine.run();
}