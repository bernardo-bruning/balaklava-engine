use glutin::{WindowBuilder};
use crate::Application;
use crate::backend::Backend;
use crate::graphics::{Bindable, Texture, ShaderProgram};
extern crate gfx_device_gl as back;
use gfx::Factory;
use gfx::format::Rgba8;

struct Graphics {
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    depth_view: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
}

impl Graphics {
    fn new() -> Graphics {
        let builder = WindowBuilder::new();
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
            .with_vsync(true);
        let events_loop = glutin::EventsLoop::new();

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
            depth_view
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

struct Gfx {
    graphics: Graphics
}

impl Gfx {
    fn new() -> Self {
        Gfx {
            graphics: Graphics::new()
        }
    }
}

impl Backend<TextureResource, Graphics> for Gfx {
    fn graphics() -> Graphics {
        return Graphics::new()
    }
}

pub fn launch<A>() where A: Application {
    let grahics = Gfx::new();
    let mut app = A::new(grahics);
    app.create();
    loop {
        app.render();
    }
}