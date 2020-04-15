extern crate balaklava;
use balaklava::{Application, lauch_gl};
use balaklava_gl::GlDevice;
use balaklava_gpu::{Device, Vector};
use std::io::Cursor;

struct Game <D: Device> {
    program: D::Program,
    vertices: D::Buffer,
    texture: D::Texture
}

impl<D: Device> Application<D> for Game<D> {
    fn new(device: &mut D) -> Self {
        let vertex_shader = include_bytes!("shader/shader_150.glslv");
        let pixel_shader = include_bytes!("shader/shader_150.glslf");
        let mut vertices = Vec::new();
        vertices.push(Vector::new(0.0, 1.0, 0.0));
        vertices.push(Vector::new(-1.0, -1.0, 0.0));
        vertices.push(Vector::new(1.0, -1.0, 0.0));
        let mut program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec());
        let buffer = device.create_vertex_buffer(&mut program, vertices);
        let texture = device.create_texture(Cursor::new(&include_bytes!("texture.png")[..]));

        Game {
            program: program,
            vertices: buffer,
            texture: texture
        }
    }
    
    fn render(&mut self, device: &mut D) {
        device.render_program(&self.program, &self.vertices, Option::None, Option::Some(&self.texture));
    }
}

fn main() {
    lauch_gl::<Game<GlDevice>>();
}