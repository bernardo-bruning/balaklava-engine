extern crate balaklava;
use balaklava::gpu_gfx::config::Config;
use balaklava::gpu_gfx::GfxDevice;
use balaklava::gpu::{Device};
use balaklava::backend::{Backend, Handle};
use balaklava::graphics::ShaderProgram;
use balaklava::Application;
use nalgebra::Vector3;
use std::*;


fn main() {
    let config = Config::default();
    let mut device = GfxDevice::new(config);
    let vertex_shader = include_bytes!("shader/shader_150.glslv");
    let pixel_shader = include_bytes!("shader/shader_150.glslf");
    let mut vertices = Vec::new();
    vertices.push(Vector3::new(0.0, 1.0, 0.0));
    vertices.push(Vector3::new(-1.0, -1.0, 0.0));
    vertices.push(Vector3::new(1.0, -1.0, 0.0));

    let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec(), vertices);
    loop {
        device.render_program(&program);
        device.flush();
    }
    // let mut backend = GfxBackend::default();
    // backend.launch(SquareApplication::default());
}