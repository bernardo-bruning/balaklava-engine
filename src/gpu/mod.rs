
use nalgebra::Vector3;

pub type Vector = Vector3<f32>;
pub type Handle = u32;
pub type Program = Handle;

pub trait Device {
    fn create_program(vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Program;
    fn render_program(program: Program);
}