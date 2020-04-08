extern crate glium;

use glium::{Display, Frame};
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
use glium::glutin::event_loop::EventLoop;
use balaklava_gpu::{Device, Vector};

pub struct Program {
    inner_program: glium::Program,
    vertices: Vec<Vector>
}

pub struct GlDevice {
    display: Display,
    frame: Frame
}

impl GlDevice {
    pub fn new<T: 'static>(events_loop: &EventLoop<T>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("balaklava engine");
        
        let context_builder = ContextBuilder::new();

        let display_result = Display::new(window_builder, context_builder, &events_loop);
        let display = display_result.unwrap();
        let frame = display.draw();
        GlDevice{
            display,
            frame,
        }
    }
}

impl Device for GlDevice {
    type Program = Program;

    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>, vertices: Vec<Vector>) -> Self::Program {
        let vertex = std::str::from_utf8(vertex_shader.as_ref()).unwrap();
        let pixel = std::str::from_utf8(pixel_shader.as_ref()).unwrap();
        let program_result = glium::Program::from_source(&self.display, vertex, pixel, None);
        return Program {
            inner_program: program_result.unwrap(),
            vertices: vertices
        }
    }

    fn render_program(&mut self, program: &Self::Program) {
        
        unimplemented!();
    }

    fn flush(&mut self) {
        self.frame.set_finish().unwrap();
        self.frame = self.display.draw();
    }
}