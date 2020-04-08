extern crate glium;

use balaklava_gpu::{Device, Vector};

pub struct Program {

}

pub struct GlDevice {}

impl GlDevice {
    fn new() -> Self {
        GlDevice{

        }
    }
}

impl Device for GlDevice {
    type Program = Program;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program {
        unimplemented!();
    }

    fn render_program(&mut self, program: &Self::Program) {
        unimplemented!();
    }

    fn flush(&mut self) {
        unimplemented!();
    }
}