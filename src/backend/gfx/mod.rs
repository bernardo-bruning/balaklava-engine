mod pool;

extern crate gfx;
extern crate gfx_device_gl as back;
use glutin::{WindowBuilder};
use gfx::Encoder;
use crate::Application;
use crate::graphics::{ShaderProgram};
use crate::backend::{Handle, Binder};
use glutin::{EventsLoop, Event, WindowEvent, ContextBuilder};


gfx_defines!{
    vertex Vertex {
        position: [f32; 4] = "vertex_position",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<gfx::format::Srgba8> = "target",
        depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub struct Config {
    title: String,
    dimension_width: usize,
    dimension_height: usize,
    fullscreen: bool
}

impl Config {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_dimensions(mut self, width: usize, height: usize) -> Self {
        self.dimension_width = width;
        self.dimension_height = height;
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Balaklava Engine".to_string(),
            dimension_height: 600,
            dimension_width: 800,
            fullscreen: false
        }
    }
}


struct Graphic {
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    depth_view: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
    events_loop: EventsLoop,
    encoder: Encoder<back::Resources, back::CommandBuffer>
}

impl Graphic {
    fn new(config: Config) -> Graphic {
        let events_loop = glutin::EventsLoop::new();
        let mut builder = WindowBuilder::new()
            .with_title(config.title)
            .with_dimensions(config.dimension_width as u32, config.dimension_height as u32);

        if config.fullscreen {
              builder = builder.with_fullscreen(Some(events_loop.get_primary_monitor()))
        }

        let context = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
            .with_vsync(true);


        let (window, device, mut factory, color, depth_view) =
            gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
            builder, 
            context, 
            &events_loop
        );

        let encoder: Encoder<back::Resources, back::CommandBuffer> = 
            factory.create_command_buffer().into();

        Graphic{
            window,
            device,
            factory,
            color,
            depth_view,
            events_loop,
            encoder
        }
    }
}

struct TextureResource {
    shaderResourceView: gfx::handle::ShaderResourceView<back::Resources, [f32; 4]>
} 

impl crate::backend::Graphic 
    for Graphic {}

impl Binder<ShaderProgram> for Graphic {
    fn bind(&mut self, bindable: ShaderProgram) -> Handle<ShaderProgram> {
        return Handle {
            identifier: 0,
            type_marker: std::marker::PhantomData
        };
    }
}

pub struct Backend {
    graphic: Graphic
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Backend {
            graphic: Graphic::new(config)
        }
    }


    pub fn launch<A>(&mut self, mut application: A) where A: Application {
        let mut running = true;
        while running {
            self.graphic.events_loop.poll_events(|event|
                match event {
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::Closed => running = false,
                        _ => ()
                    },
                    _ => ()
                }
            );
            application.run(self);
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        return Backend::new(Config::default())
    }
}

impl crate::backend::Backend for Backend {
    fn graphic(&mut self) -> &mut dyn crate::backend::Graphic {
        &mut self.graphic
    }
}