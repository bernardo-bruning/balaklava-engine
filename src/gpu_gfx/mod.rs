pub mod config;
mod pipeline;

extern crate gfx;
extern crate gfx_device_gl as back;

use crate::gpu::{Vector, Device};
use glutin::{WindowBuilder};
use gfx::{Encoder};
use gfx::traits::FactoryExt;
use glutin::{ContextBuilder};

pub struct Program {
    pub data: pipeline::pipe::Data<back::Resources>,
    pub slice: gfx::Slice<back::Resources>,
    pub pso: gfx::PipelineState<back::Resources, pipeline::pipe::Meta>
}

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
    type Program = Program;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Program {
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&pipeline::as_vertex(vertices), ());
        let pso = self.factory.create_pipeline_simple(
            vertex_shader.as_ref(), 
            pixel_shader.as_ref(),
            pipeline::pipe::new()
        );

        if pso.is_err() {
            panic!("Error to load pso! {:?}", pso.err());
        }

        let data = pipeline::pipe::Data {
            vbuf: vertex_buffer,
            out: self.color.clone(),
            depth: self.depth_view.clone()
        };

        return Program {
            data,
            slice,
            pso: pso.unwrap()
        }
    }
    
    fn render_program(&mut self, program: Program) {
        unimplemented!();
    }
}