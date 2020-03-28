use glutin::{WindowBuilder};
use crate::Application;
use crate::graphics::{Bindable, Texture, ShaderProgram};
use glutin::{EventsLoop, Event, WindowEvent, ContextBuilder};
extern crate gfx_device_gl as back;

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
            dimension_height: 800,
            dimension_width: 500,
            fullscreen: false
        }
    }
}


struct Graphics {
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    depth_view: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
    events_loop: EventsLoop
}

impl Graphics {
    fn new(config: Config) -> Graphics {
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


        let (window, device, factory, color, depth_view) =
            gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
            builder, 
            context, 
            &events_loop
        );

        Graphics{
            window,
            device,
            factory,
            color,
            depth_view,
            events_loop
        }
    }
}

struct TextureResource {
    shaderResourceView: gfx::handle::ShaderResourceView<back::Resources, [f32; 4]>
} 

impl crate::backend::Graphics<TextureResource> 
    for Graphics {}

impl Bindable<Texture<TextureResource>> for Graphics {
    fn bind(&mut self, bindable: &mut Texture<TextureResource>) -> bool {
        unimplemented!()
    }
}

impl Bindable<ShaderProgram> for Graphics {
    fn bind(&mut self, bindable: &mut ShaderProgram) -> bool {
        unimplemented!()
    }
}

pub struct Backend {
    graphics: Graphics
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Backend {
            graphics: Graphics::new(config)
        }
    }

    pub fn with_title(&mut self, title: String) {
        println!("fix me: with_ttitle not implemented!");
    }

    pub fn launch<A>(&mut self, application: &mut A) where A: Application {
        let mut running = true;
        application.create(self);
        while running {
            self.graphics.events_loop.poll_events(|event|
                match event {
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::Closed => running = false,
                        _ => ()
                    },
                    _ => ()
                }
            );
            application.render();
        }
    }
}

impl crate::backend::Backend<TextureResource, Graphics> for Backend {
    fn graphics() -> Graphics {
        unimplemented!();
    }
}