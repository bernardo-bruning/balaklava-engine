#[macro_use]
extern crate gfx;
extern crate nalgebra as na;
mod core;
mod geometry;
mod camera;
mod loaders;
use crate::geometry::{Triangle, Renderable};
use crate::core::*;

fn main() {
    let builder = core::Builder::default()
        .with_name("Learn GFX".to_string())
        .with_light(Light {
            position: [0.5, 0.0, 0.07, 1.0],
            color: [1.0, 1.0, 1.0]
        });

    let mut triangle = Triangle::new();
    let mut engine = builder.build();

    let mut running = true;
    while running {
        engine.camera.translate(0., 0., -0.01);
        engine.poll_event(|event| match event {
            Event::Closed => running = false
        });

        engine.clear();
        triangle.render(&mut engine);
        engine.update();
    }
}