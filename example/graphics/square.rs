extern crate balaklava;
use balaklava_gfx::config::Config;
use balaklava_gfx::GfxDevice;
use balaklava_gpu::{Device, Vector};
use balaklava_gfx::glutin::{Event, WindowEvent, EventsLoop};
use std::*;


fn main() {
    let config = Config::default();
    let mut events_loop = EventsLoop::new();
    let mut device = GfxDevice::new(config, &events_loop);
    let vertex_shader = include_bytes!("shader/shader_150.glslv");
    let pixel_shader = include_bytes!("shader/shader_150.glslf");
    let mut vertices = Vec::new();
    vertices.push(Vector::new(0.0, 1.0, 0.0));
    vertices.push(Vector::new(-1.0, -1.0, 0.0));
    vertices.push(Vector::new(1.0, -1.0, 0.0));

    let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec(), vertices);
    let mut running = true;

    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Closed => running = false,
                _ => ()
            },
            _ => ()
        });

        device.render_program(&program);
        device.flush();
    }
}