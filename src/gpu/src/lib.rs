extern crate nalgebra;
use nalgebra::{Vector3, Matrix4};
use std::io::{BufRead, Seek};

pub type Vector = Vector3<f32>;
pub type Transform = Matrix4<f32>;

pub trait Device {
    type Program;
    type Buffer;
    type Texture;
    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program;
    fn create_vertex_buffer(&mut self, program: &mut Self::Program, vertices: Vec<Vector>) -> Self::Buffer;
    fn create_texture<R: BufRead+Seek>(&mut self, reader: R) -> Self::Texture;
    fn render_program(&mut self, program: &Self::Program, buffer: &Self::Buffer, transform: Transform, texture: &Self::Texture);
    fn flush(&mut self);
}