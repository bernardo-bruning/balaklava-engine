extern crate balaklava;
use balaklava::backend::gfx::Backend as GfxBackend;
use balaklava::backend::{Backend};
use balaklava::graphics::ShaderProgram;
use balaklava::Application;
use nalgebra::Vector3;
use std::*;

struct SquareApplication {

}

impl SquareApplication {
    fn new() -> Self {
        let vertex_shader = include_bytes!("shader/shader_150.glslf");
        let pixel_shader = include_bytes!("shader/shader_150.glslv");
        let mut shader_program = ShaderProgram::new(
            vertex_shader.to_vec(), 
            pixel_shader.to_vec()
        );

        shader_program.vertices.push(Vector3::new(0.0, 0.5, 0.0));
        shader_program.vertices.push(Vector3::new(1.0, 0.0, 0.0));
        shader_program.vertices.push(Vector3::new(1.0, 1.0, 0.0));
        
        Self{}
    }
}

impl Application for SquareApplication {
    fn run(&mut self, backend: &mut dyn Backend){
        
    }
}

fn main() {
    let mut backend = GfxBackend::default();
    backend.launch(SquareApplication{});
}