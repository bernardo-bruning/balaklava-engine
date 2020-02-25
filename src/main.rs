#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl as back;
extern crate nalgebra as na;

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

    constant Viewport {
        transform: [[f32; 4]; 4] = "viewport_tranform",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        light: gfx::ConstantBuffer<Light> = "light",
        viewport: gfx::ConstantBuffer<Viewport> = "viewport",
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

enum Event {
    Closed,
}

struct Mesh<'a> {
    vertices: &'a[Vertex],
    index: Option<gfx::Slice<back::Resources>>,
    data: Option<pipe::Data<back::Resources>>
}

impl <'a> Mesh<'a> {
    fn new(vertices: &'a[Vertex]) -> Self {
        return Mesh{
            vertices: vertices,
            index: Option::None,
            data: Option::None
        }
    }

    fn bind(&mut self, engine: &mut Engine) {
        let light_buffer = engine.factory.create_constant_buffer(1);
        let viewport_buffer = engine.factory.create_constant_buffer(1);
        let (vertex_buffer, index) = engine.factory.create_vertex_buffer_with_slice(self.vertices, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            light: light_buffer,
            viewport: viewport_buffer,
            out: engine.color.clone(),
        };

        self.index = Option::Some(index);
        self.data = Option::Some(data);
    }

    fn render(&mut self, engine: &mut Engine) {
        if self.data == Option::None || self.index == Option::None {
            self.bind(engine);
        }
        
        let viewport = engine.viewport.unwrap_or(Viewport{
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.6, 0.0, 0.0],
                [0.0, 0.0, 0.6, 0.0],
                [0.0, 0.0, 0.0, 0.6],
            ]
        });
        let data = self.data.as_ref().unwrap();
        let index = self.index.as_ref().unwrap();
        engine.encoder.update_buffer(&data.light, &engine.lights, 0).unwrap();
        engine.encoder.update_buffer(&data.viewport, &[viewport], 0).unwrap();
        engine.encoder.draw(index, &engine.pso, data);
    }
}

struct Engine<'a> {
    pub lights: &'a[Light],
    event_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    encoder: gfx::Encoder<back::Resources, back::CommandBuffer>,
    pso: gfx::pso::PipelineState<back::Resources, pipe::Meta>,
    viewport: Option<Viewport>
}

impl <'a> Engine<'a> {
    fn new(config: EngineConfiguration<'a>, lights: &'a[Light]) 
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
            lights,
            event_loop,
            window,
            device,
            factory,
            color,
            encoder,
            pso,
            viewport: Option::None
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

    fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = Option::from(viewport);
    }

    fn clear(&mut self) {
        self.device.cleanup();
        self.encoder.clear(&self.color, [0.0, 0.0, 0.0, 1.0]);
    }

    fn update(&mut self) {
        self.window.swap_buffers().unwrap();
        self.encoder.flush(&mut self.device);
    }
}

fn main() {
    let config = EngineConfiguration::default()
        .with_name("Learn GFX".to_string());

    let mut mesh = Mesh::new(
        &[
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
        ]
    );


    let lights = &[
        Light {
            position: [0.5, 0.0, 0.07, 1.0],
            color: [1.0, 1.0, 1.0]
        }
    ][0..1];

    let mut engine = Engine::new(config, lights)
        .unwrap();
    let mut transform = na::Matrix4::from_scaled_axis(na::base::Vector3::new(0.0, 0.0, 0.0));
    engine.set_viewport(Viewport{ transform: transform.into() });
    
    let rotation = na::Rotation3::new(na::Vector3::new(0.0, 0.0, 0.01));
    let mut running = true;
    while running {
        transform = transform * rotation.to_homogeneous();
        engine.set_viewport(Viewport{ transform: transform.into() });
        engine.poll_event(|event| match event {
            Event::Closed => running = false
        });

        engine.clear();
        mesh.render(&mut engine);
        engine.update();
    }
}