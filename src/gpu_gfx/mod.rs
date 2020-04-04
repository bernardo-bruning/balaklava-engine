pub mod config;

extern crate gfx;
extern crate gfx_device_gl as back;

use crate::gpu::{Vector, Device, Program};
use glutin::{WindowBuilder};
use gfx::{Encoder};
use glutin::{ContextBuilder};

pub struct GfxDevice {
    window: glutin::GlWindow,
    device: back::Device,
    factory: back::Factory,
    color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    depth_view: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
    events_loop: glutin::EventsLoop,
    encoder: Encoder<back::Resources, back::CommandBuffer>
}

impl GfxDevice {
    pub fn new(config: config::Config) -> Self {
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

        GfxDevice{
            window,
            device,
            factory,
            color,
            depth_view,
            events_loop,
            encoder,
        }
    }
}

impl Device for GfxDevice {
    fn create_program(vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Program {
        unimplemented!();
    }
    
    fn render_program(program: Program) {
        unimplemented!();
    }
}