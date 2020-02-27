#[macro_use]
extern crate gfx;
extern crate nalgebra as na;
mod core;
mod geometry;

use na::{Vector3,Rotation3, Point3, Translation3, Orthographic3};
use crate::geometry::Mesh;
use crate::core::*;

fn main() {
    let config = EngineConfiguration::default()
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

    let projection = Orthographic3::new(-10., 10., -10., 10., 0., 100.).to_homogeneous();
    let mut view = na::Matrix4::look_at_rh(
        &Point3::new(0., 0., 1.), 
        &Point3::new(0., 0., 0.), 
        &Vector3::new(0., 1., 0.)
    );

    let mut camera = projection * view;
    engine.set_camera(camera);
    let translation = Translation3::new(0., 0., -0.01).to_homogeneous();

    let rotation = Rotation3::new(Vector3::new(0., 0.01, 0.0)).to_homogeneous();
    
    let mut running = true;
    while running {
        camera = camera * translation ;
        engine.set_camera(camera);
        
        engine.poll_event(|event| match event {
            Event::Closed => running = false
        });

        engine.clear();
        mesh.render(&mut engine);
        engine.update();
    }
}