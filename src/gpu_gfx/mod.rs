pub mod config;

use crate::gpu::{Vector, Device, Program};

struct GfxDevice {

}

impl GfxDevice {
    fn new(config: config::Config) -> Self {
        GfxDevice{}
    }
}

impl Device for GfxDevice {
    fn create_program(vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Program {
        unimplemented!();
    }
    
    fn render_program(program: Program) {
        unimplemented!();
    }
}