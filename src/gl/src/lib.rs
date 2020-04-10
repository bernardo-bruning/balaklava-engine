#[macro_use]
pub extern crate glium;

use glium::{Display, Frame, VertexBuffer, Surface};
use glium::index::NoIndices;
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use balaklava_gpu::{Device, Vector, Transform};
use std::rc::Rc;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4]
}

implement_vertex!(Vertex, position);

impl From<&Vector> for Vertex {
    fn from(vector: &Vector) -> Self {
        return Vertex {
            position: [vector[0], vector[1], vector[2], 1.0]
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
    inner_program: glium::Program,
    buffer: Buffer
}

pub struct GlDevice {
    display: Display,
    frame: Frame
}

impl GlDevice {
    pub fn new<T: 'static>(events_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("balaklava engine");
        
        let context_builder = ContextBuilder::new();

        let display_result = Display::new(window_builder, context_builder, &events_loop);
        let display = display_result.unwrap();
        let frame = display.draw();
        GlDevice{
            display,
            frame,
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

    fn render_program(&mut self, program: &Program, buffer: &Buffer, transform: Transform) {
        let matrix_transform: [[f32; 4]; 4] = transform.into();
        let uniforms = uniform!{ transform: matrix_transform  };
        let buffer_borrow = buffer;
        self.frame.draw(
            buffer_borrow.inner.as_ref(), 
            &buffer_borrow.indice, 
            &program.inner_program, 
            &uniforms, 
            &Default::default())
            .unwrap();
    }
}

impl Device for GlDevice {
    type Program = Program;
    type Buffer = Buffer;
    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program {
        let vertex = std::str::from_utf8(vertex_shader.as_ref()).unwrap();
        let pixel = std::str::from_utf8(pixel_shader.as_ref()).unwrap();
        let program_result = glium::Program::from_source(&self.display, vertex, pixel, None);
        let buffer = self.create_vertex_buffer(vertices);
        return Program {
            inner_program: program_result.unwrap(),
            buffer
        };
    }

    fn create_vertex_buffer(&mut self, program: &mut Self::Program, vertices: Vec<Vector>) -> Self::Buffer {
        let buffer = self.create_vertex_buffer(vertices);
        program.buffer = buffer.clone();
        return buffer;
    }

    fn render_program(&mut self, program: &Program) {
        self.render_program(program, &program.buffer, Transform::identity());
    }

    fn flush(&mut self) {
        self.frame.set_finish().unwrap();
        self.frame = self.display.draw();
        self.frame.clear_color(0.1, 0.2, 0.3, 1.0)
    }
}