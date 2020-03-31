extern crate balaklava;
use balaklava::backend::gfx::Backend as GfxBackend;
use balaklava::backend::{Backend, Handle};
use balaklava::graphics::ShaderProgram;
use balaklava::Application;
use nalgebra::Vector3;
use std::*;

struct SquareApplication {
    triangle: Option<Handle<ShaderProgram>>
}

impl SquareApplication {
    fn create_shader(&self) -> ShaderProgram {
        let vertex_shader = include_bytes!("shader/shader_150.glslv");
        let pixel_shader = include_bytes!("shader/shader_150.glslf");
        let mut shader_program = ShaderProgram::new(
            vertex_shader.to_vec(), 
            pixel_shader.to_vec()
        );

        shader_program.vertices.push(Vector3::new(0.0, 1.0, 0.0));
        shader_program.vertices.push(Vector3::new(-1.0, -1.0, 0.0));
        shader_program.vertices.push(Vector3::new(1.0, -1.0, 0.0));
        shader_program
    }
}

impl Default for SquareApplication {
    fn default() -> Self {        
        Self{
            triangle: Option::None
        }
    }
}

impl Application for SquareApplication {
    fn run(&mut self, backend: &mut dyn Backend){
        let graphic = backend.graphic();
        match &self.triangle {
            Option::None => self.triangle = Option::Some(graphic.bind(self.create_shader())),
            Option::Some(handle) => graphic.render(handle),
            _ => ()
        }
    }
}

fn main() {
    let mut backend = GfxBackend::default();
    backend.launch(SquareApplication::default());
}