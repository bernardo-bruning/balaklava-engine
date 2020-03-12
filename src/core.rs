extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl as back;
extern crate nalgebra as na;

use crate::camera;
use crate::events;
use gfx::traits::FactoryExt;
use gfx::{Device, Factory};
use glutin::{GlContext};

gfx_defines!{
    vertex Vertex {
        position: [f32; 4] = "vertex_position",
        normal: [f32; 3] = "vertex_normal",
        color: [f32; 3] = "vertex_color",
        uv: [f32; 2] = "vertex_uv",
    }

    constant Light {
        position: [f32; 4] = "light_position",
        color: [f32; 3] = "light_color",
    }

    constant Camera {
        transform: [[f32; 4]; 4] = "camera_tranform",
        projection: [[f32; 4]; 4] = "camera_projection",
    }

    constant Transform {
        matrix: [[f32; 4]; 4] = "transform_matrix",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        light: gfx::ConstantBuffer<Light> = "light",
        transformation: gfx::ConstantBuffer<Transform> = "transform",
        camera: gfx::ConstantBuffer<Camera> = "camera",
        texture: gfx::TextureSampler<[f32; 4]> = "t_texture",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "target",
        depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub struct Builder<'a> {
    name: String,
    vertex_shader: &'a[u8],
    pixel_shader: &'a[u8],
    light: Option<Light>
}

impl <'a> Builder<'a> {
    pub fn default() -> Builder<'a> {
        return Self{
            name: "".to_string(),
            vertex_shader: include_bytes!("shader/shader_150.glslv"),
            pixel_shader: include_bytes!("shader/shader_150.glslf"),
            light: Option::None
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    } 

    pub fn with_vertex_shader(mut self, shader: &'a[u8]) -> Self {
        self.vertex_shader = shader;
        self
    }

    pub fn with_pixel_shader(mut self, shader: &'a[u8]) -> Self {
        self.pixel_shader = shader;
        self
    }

    pub fn with_light(mut self, light: Light) -> Self{
        self.light = Option::Some(light);
        self
    }
    
    fn get_lights(&self) -> Vec<Light>{
        if self.light.is_some() {
            let light = self.light.unwrap();
            return vec!(light);
        }
        return vec!();
    }

    fn get_eventsloop(&self) -> glutin::EventsLoop {
        return glutin::EventsLoop::new();
    }

    fn get_window_builder(&self) -> glutin::WindowBuilder {
        glutin::WindowBuilder::new()
            .with_dimensions(500, 400)
            .with_title(self.name.clone())
    }

    fn get_context_builder(&self) -> glutin::ContextBuilder {
        glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3,2)))
            .with_vsync(true)
    }

    pub fn build(&self) -> Engine {            
        let events_loop = self.get_eventsloop();
        let (window, device, mut factory, color, depth_view) =
            gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(
            self.get_window_builder(), 
            self.get_context_builder(), 
            &events_loop
        );

        let pso = factory.create_pipeline_simple(
            self.vertex_shader,
            self.pixel_shader,
            pipe::new()
        ).unwrap();

        let encoder: gfx::Encoder<back::Resources, back::CommandBuffer> = 
            factory.create_command_buffer().into();
            
        return Engine::new(
            self.get_lights(),
            events_loop,
            window,
            device,
            factory,
            color,
            depth_view,
            encoder,
            pso,
            camera::Orthographic::default()
        )
    } 
}

#[derive(Debug, Clone)]
pub struct Texture {
    data: Vec<u8>,
    width: u16,
    height: u16,
    pub resource: Option<gfx::handle::ShaderResourceView<back::Resources, [f32; 4]>>
}

impl Texture {
    pub fn load(path: String) -> Result<Self, String>{
        let image_result = image::open(path);
        if image_result.is_err() {
            return Result::Err("Error to load data!".to_string())
        }
        let image = image_result.unwrap().to_rgba();
        log::info!("image loaded has size {}", image.len());
        let (width, height) = image.dimensions();
        let texture = Texture{
            width: width as u16,
            height: height as u16,
            data: image.to_vec(),
            resource: Option::None
        };
        
        return Result::Ok(texture);
    }
}

impl Default for Texture {
    fn default() -> Self{
        return Texture{
            width: 1,
            height: 1,
            data: vec![0],
            resource: Option::None
        }
    }
}

pub trait Bindable<T> {
    fn bind(&mut self, t:T) -> Result<T, String>;
}

pub struct Engine {
    pub lights: Vec<Light>,
    event_loop: glutin::EventsLoop,
    pub window: glutin::GlWindow,
    device: back::Device,
    pub factory: back::Factory,
    pub depth: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
    pub color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
    pub encoder: gfx::Encoder<back::Resources, back::CommandBuffer>,
    pub pso: gfx::pso::PipelineState<back::Resources, pipe::Meta>,
    pub camera: camera::Orthographic
}

impl Engine {
    pub fn new(
        lights: Vec<Light>,
        event_loop: glutin::EventsLoop,
        window: glutin::GlWindow,
        device: back::Device,
        factory: back::Factory,
        color: gfx::handle::RenderTargetView<back::Resources, gfx::format::Srgba8>,
        depth: gfx::handle::DepthStencilView<back::Resources, gfx::format::DepthStencil>,
        encoder: gfx::Encoder<back::Resources, back::CommandBuffer>,
        pso: gfx::PipelineState<back::Resources, pipe::Meta>,
        camera: camera::Orthographic
    ) -> Self {
        return Engine {
            lights,
            event_loop,
            window,
            device,
            factory,
            color,
            depth,
            encoder,
            pso,
            camera: camera
        }
    }

    pub fn poll_event<F>(&mut self, mut callback: F) where F:FnMut(events::Event) {
        self.event_loop.poll_events(|event| {
            let event = events::convert(event);
            if event.is_some() {
                callback(event.unwrap());
            }
        });
    }

    pub fn clear(&mut self) {
        self.device.cleanup();
        self.encoder.reset();
        self.encoder.clear(&self.color, [1.0, 1.0, 1.0, 1.0]);
        self.encoder.clear_depth(&self.depth, 1.0);
    }

    pub fn update(&mut self) {
        self.window.swap_buffers().unwrap();
        self.encoder.flush(&mut self.device);
    }
}

impl Bindable<Texture> for Engine {
    fn bind(&mut self, texture: Texture) -> Result<Texture, String> {
        use gfx::format::Rgba8;
        let kind = gfx::texture::Kind::D2(texture.width, texture.height, gfx::texture::AaMode::Single);
        let result = self.factory.create_texture_immutable_u8::<Rgba8>(kind, gfx::texture::Mipmap::Provided, &[&texture.data]);
        if result.is_err() {
            return Result::Err("Erro to create texture".to_string());
        }
        let (_, resource) = result.unwrap();
        return Result::Ok(Texture{
            width: texture.width,
            height: texture.height,
            data: texture.data,
            resource: Option::Some(resource)
        });
    }
}