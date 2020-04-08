
extern crate nalgebra;
use nalgebra::Vector3;

pub type Vector = Vector3<f32>;

pub trait Device {
    type Program;
    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program;
    fn create_vertex_buffer(&mut self, program: &Self::Program, vertices: Vec<Vector>);
    fn render_program(&mut self, program: &Self::Program);
    fn flush(&mut self);
}