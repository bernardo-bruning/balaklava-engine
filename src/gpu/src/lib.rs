extern crate nalgebra;
use balaklava_math::{Vector, Transform};

pub trait Device {
    type Program;
    type Buffer;
    type Texture;
    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>) -> Self::Program;
    fn create_vertex_buffer(&mut self, program: &mut Self::Program, vertices: Vec<Vector>) -> Self::Buffer;
    fn create_texture(&mut self, data: Vec<u8>, dimensions: Vector) -> Self::Texture;
    fn render_program(&mut self, program: &Self::Program, buffer: &Self::Buffer, transform: Option<Transform>, texture: Option<&Self::Texture>);
    fn flush(&mut self);
}