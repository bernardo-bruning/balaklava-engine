#[macro_use]
extern crate gfx;
extern crate nalgebra as na;
mod core;
mod geometry;
mod camera;

use na::{Vector3,Rotation3, Point3, Translation3, Orthographic3};
use crate::geometry::Mesh;
use crate::core::*;
use crate::camera::Orthographic;

fn main() {
    let builder = core::Builder::default()
        .with_name("Learn GFX".to_string());

    let mut mesh = Mesh::new(
        &[
            Vertex { 
                position: [ -0.5, -0.5, 1.0, 1.0 ], 
                normal: [0.0, 0.0, 1.0], 
                color: [1.0, 0.0, 0.0] 
            },
            Vertex { 
                position: [  0.5, -0.5, 1.0, 1.0 ], 
                normal: [0.0, 0.0, 1.0], 
                color: [0.0, 1.0, 0.0] 
            },
            Vertex { 
                position: [  0.0,  0.5, 1.0, 1.0 ], 
                normal: [0.0, 0.0, 1.0], 
                color: [0.0, 0.0, 1.0] 
            },
        ]
    );

    let lights = &[
        Light {
            position: [0.5, 0.0, 0.07, 1.0],
            color: [1.0, 1.0, 1.0]
        }
    ][0..1];

    let mut engine = Engine::new(config, lights)
        .unwrap();

    let mut running = true;
    while running {
        engine.camera.translate(0., 0., -0.01);
        engine.poll_event(|event| match event {
            Event::Closed => running = false
        });

        engine.clear();
        mesh.render(&mut engine);
        engine.update();
    }
}