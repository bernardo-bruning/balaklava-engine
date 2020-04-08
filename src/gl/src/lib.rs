extern crate glium;

use glium::glutin::event_loop::EventLoop;
use balaklava_gpu::{Device, Vector};

pub struct Program {

}

pub struct GlDevice {
    display: glium::Display
}

impl GlDevice {
    pub fn new<T: 'static>(events_loop: &EventLoop<T>) -> Self {
        let window_builder = glium::glutin::window::WindowBuilder::new()
            .with_title("balaklava engine");
        
        let context_builder = glium::glutin::ContextBuilder::new();

        let display_result = glium::Display::new(window_builder, context_builder, &events_loop);
        let display = display_result.unwrap();

        GlDevice{
            display
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