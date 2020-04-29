#[macro_use]
pub extern crate glium;

use glium::{Display, Frame, VertexBuffer, Surface};
use glium::index::NoIndices;
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use balaklava_math::{Vector, Transform, Camera};
use balaklava_gpu::{Device};
use glium::texture::Texture2d;
use std::rc::Rc;

#[derive(Copy, Clone)]
struct GlVertex {
    position: [f32; 4],
    texture_region: [f32; 3]
}

implement_vertex!(GlVertex, position, texture_region);

impl From<&Vector> for GlVertex {
    fn from(vector: &Vector) -> Self {
        return GlVertex {
            position: [vector[0], vector[1], vector[2], 1.0],
            texture_region: [vector[0], vector[1], 1.0]
        }
    }
}

impl From<&(Vector, Option<Vector>)> for GlVertex {
    fn from(vector: &(Vector, Option<Vector>)) -> Self {
        let position = vector.0;
        let texture_region = vector.1.unwrap_or(vector.0);
        return GlVertex {
            position: [position[0], position[1], position[2], 1.0],
            texture_region: [texture_region[0], texture_region[1], 1.0]
        }
    }
}

#[derive(Clone)]
pub struct Buffer {
    inner: Rc<VertexBuffer<GlVertex>>,
    indice: NoIndices,
}

impl Buffer {
    fn new(buffer: VertexBuffer<GlVertex>, indice: NoIndices) -> Self {
        Buffer { 
            inner: Rc::from(buffer), 
            indice: indice
        }
    }
}

pub struct Program {
    inner_program: glium::Program
}

pub struct Texture {
    inner: Texture2d
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

    fn create_vertex_buffer_with_vertex(&mut self, vertex: Vec<(Vector, Option<Vector>)>) -> Buffer {
        let gl_vertex: Vec<GlVertex> = vertex.iter()
            .map(|vertex| GlVertex::from(vertex))
            .collect();
        let vertex_buffer_result = VertexBuffer::new(&self.display, &gl_vertex);
        let indices = NoIndices(glium::index::PrimitiveType::TrianglesList);
        return Buffer::new(vertex_buffer_result.unwrap(), indices);
    }

    fn create_vertex_buffer(&mut self, vertices: Vec<Vector>, texture_regions: Vec<Vector>) -> Buffer {        
        let vertex: Vec<(Vector, Option<Vector>)> = texture_regions
            .iter()
            .enumerate()
            .map(|(i, texture_region)| (vertices[i], Option::Some(texture_region.clone())))
            .collect();
        return self.create_vertex_buffer_with_vertex(vertex);
    }
}

impl Device for GlDevice {
    type Program = Program;
    type Buffer = Buffer;
    type Texture = Texture;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>) -> Self::Program {
        let vertex = std::str::from_utf8(vertex_shader.as_ref()).unwrap();
        let pixel = std::str::from_utf8(pixel_shader.as_ref()).unwrap();
        let program_result = glium::Program::from_source(&self.display, vertex, pixel, None);
        return Program {
            inner_program: program_result.unwrap()

        };
    }

    fn create_vertex_buffer(&mut self, _program: &mut Self::Program, vertices: Vec<Vector>, mut texture_region: Option<Vec<Vector>>) -> Self::Buffer {
        if texture_region.is_none() {
            texture_region = Option::Some(vertices.clone());
        }
        let buffer = self.create_vertex_buffer(vertices, texture_region.unwrap());
        return buffer;
    }

    fn create_texture(&mut self, data: Vec<u8>, dimensions: Vector) -> Self::Texture {
        let dimensions = (dimensions[0] as u32, dimensions[1] as u32);
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&data, dimensions);
        let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();
        return Texture {
            inner: texture
        }
    }

    fn render_program(&mut self, program: &Program, buffer: &Buffer, transform: Option<Transform>, texture: Option<&Texture>) {
        let mut transform = transform;
        if transform.is_none() {
            transform = Option::Some(Transform::default());
        }

        let texture = match texture {
            Some(texture) => &texture.inner,
            None => &self.empty_texture
        };

        let camera = Camera::from((800., 600.));
        let transform = camera*transform.unwrap();
        let matrix_transform: [[f32; 4]; 4] = transform.into();
        let uniforms = uniform!{ transform: matrix_transform, sampler_texture: texture };
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