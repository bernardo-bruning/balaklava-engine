#[macro_use]
pub extern crate glium;

use glium::{Display, Frame, VertexBuffer, Surface};
use glium::index::NoIndices;
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use balaklava_gpu::{Device, Vector, Transform};
use glium::texture::Texture2d;
use std::io::{BufRead, Seek};
use std::rc::Rc;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
    texture_region: [f32; 3]
}

implement_vertex!(Vertex, position, texture_region);

impl From<&Vector> for Vertex {
    fn from(vector: &Vector) -> Self {
        return Vertex {
            position: [vector[0], vector[1], vector[2], 1.0],
            texture_region: [vector[0], vector[1], 1.0]
        }
    }
}

#[derive(Clone)]
pub struct Buffer {
    inner: Rc<VertexBuffer<Vertex>>,
    indice: NoIndices,
}

impl Buffer {
    fn new(buffer: VertexBuffer<Vertex>, indice: NoIndices) -> Self {
        Buffer { 
            inner: Rc::from(buffer), 
            indice: indice
        }
    }
}

pub struct Program {
    inner_program: glium::Program
}

pub struct GlDevice {
    display: Display,
    frame: Frame,
    empty_texture: Texture2d
}

impl GlDevice {
    pub fn new<T: 'static>(events_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("balaklava engine");
        
        let context_builder = ContextBuilder::new();

        let display_result = Display::new(window_builder, context_builder, &events_loop);
        let display = display_result.unwrap();
        let frame = display.draw();
        let empty_texture = Texture2d::empty(&display, 1, 1).unwrap();
        GlDevice{
            display,
            frame,
            empty_texture 
        }
    }

    fn create_vertex_buffer(&mut self, vertices: Vec<Vector>) -> Buffer {
        let vertex: Vec<Vertex> = vertices
            .iter()
            .map(|vertice| Vertex::from(vertice)).collect();

        let vertex_buffer_result = VertexBuffer::new(&self.display, vertex.as_ref());
        let indices = NoIndices(glium::index::PrimitiveType::TrianglesList);
        return Buffer::new(vertex_buffer_result.unwrap(), indices);
    }
}

impl Device for GlDevice {
    type Program = Program;
    type Buffer = Buffer;
    type Texture = Texture2d;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>) -> Self::Program {
        let vertex = std::str::from_utf8(vertex_shader.as_ref()).unwrap();
        let pixel = std::str::from_utf8(pixel_shader.as_ref()).unwrap();
        let program_result = glium::Program::from_source(&self.display, vertex, pixel, None);
        return Program {
            inner_program: program_result.unwrap()

        };
    }

    fn create_vertex_buffer(&mut self, _program: &mut Self::Program, vertices: Vec<Vector>) -> Self::Buffer {
        let buffer = self.create_vertex_buffer(vertices);
        return buffer;
    }

    fn create_texture<R: BufRead+Seek>(&mut self, reader: R) -> Self::Texture {
        let image = image::load(reader, image::ImageFormat::Png)
            .unwrap().to_rgb();
        let image_dimension = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimension);
        let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();
        return texture
    }

    fn render_program(&mut self, program: &Program, buffer: &Buffer, transform: Option<Transform>, texture: Option<&Texture2d>) {
        let mut transform = transform;
        let mut texture = texture;
        if transform.is_none() {
            transform = Option::Some(Transform::default());
        }

        if texture.is_none() {
            texture = Option::Some(&self.empty_texture);
        }
        let matrix_transform: [[f32; 4]; 4] = transform.unwrap().into();
        let uniforms = uniform!{ transform: matrix_transform, sampler_texture: texture.unwrap()  };
        let buffer_borrow = buffer;
        self.frame.draw(
            buffer_borrow.inner.as_ref(), 
            &buffer_borrow.indice, 
            &program.inner_program, 
            &uniforms, 
            &Default::default())
            .unwrap();
    }

    fn flush(&mut self) {
        self.frame.set_finish().unwrap();
        self.frame = self.display.draw();
        self.frame.clear_color(0.1, 0.2, 0.3, 1.0)
    }
}