extern crate balaklava;
use balaklava::{Application, lauch_gfx};
use balaklava_gfx::config::Config;
use balaklava_gfx::{GfxDevice};
use balaklava_gpu::{Device, Vector};

struct Game <D: Device> {
    program: D::Program
}

impl<D: Device> Application<D> for Game<D> {
    fn new(device: &mut D) -> Self {
        let vertex_shader = include_bytes!("shader/shader_150.glslv");
        let pixel_shader = include_bytes!("shader/shader_150.glslf");
        let mut vertices = Vec::new();
        vertices.push(Vector::new(0.0, 1.0, 0.0));
        vertices.push(Vector::new(-1.0, -1.0, 0.0));
        vertices.push(Vector::new(1.0, -1.0, 0.0));

        let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec(), vertices);
        
        Game {
            program
        }
    }
    
    fn render(&mut self, device: &mut D) {
        device.render_program(&self.program);
    }
}

fn main() {
    lauch_gfx::<Game<GfxDevice>>(Config::default());
}