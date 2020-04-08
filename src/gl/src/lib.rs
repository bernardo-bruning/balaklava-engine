#[macro_use]
pub extern crate glium;

use glium::{Display, Frame, VertexBuffer, Surface};
use glium::index::NoIndices;
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use balaklava_gpu::{Device, Vector};

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

pub struct Program {
    inner_program: glium::Program,
    indices: NoIndices,
    vertex_buffer: VertexBuffer<Vertex>
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
}

impl Device for GlDevice {
    type Program = Program;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program {
        let vertex = std::str::from_utf8(vertex_shader.as_ref()).unwrap();
        let pixel = std::str::from_utf8(pixel_shader.as_ref()).unwrap();
        let program_result = glium::Program::from_source(&self.display, vertex, pixel, None);
        let vertex: Vec<Vertex> = vertices
            .iter()
            .map(|vertice| Vertex::from(vertice)).collect();

        let vertex_buffer_result = VertexBuffer::new(&self.display, vertex.as_ref());
        let indices = NoIndices(glium::index::PrimitiveType::TrianglesList);
        return Program {
            inner_program: program_result.unwrap(),
            vertex_buffer: vertex_buffer_result.unwrap(),
            indices
        };
    }

    fn render_program(&mut self, program: &Program) {
        self.frame.draw(
            &program.vertex_buffer, 
            &program.indices, 
            &program.inner_program, 
            &glium::uniforms::EmptyUniforms, 
            &Default::default())
            .unwrap();
    }

    fn flush(&mut self) {
        self.frame.set_finish().unwrap();
        self.frame = self.display.draw();
        self.frame.clear_color(0.1, 0.2, 0.3, 1.0)
    }
}