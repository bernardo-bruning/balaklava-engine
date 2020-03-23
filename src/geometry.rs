extern crate gfx;
extern crate gfx_device_gl as back;
extern crate nalgebra as na;
use crate::core::*;
use na::{Matrix4,Vector3,Rotation3};
use gfx::traits::FactoryExt;
use std::ops::Deref;
use crate::core::Bindable;

pub trait Renderable {
    fn render(&mut self, engine: &mut Graphics);
}

pub struct Triangle {
    pub mesh: Mesh
}

impl Triangle {
    pub fn new() -> Self{
        return Triangle{
            mesh: Mesh::new(
                vec![
                    Vertex { 
                        position: [ -0.5, -0.5, 1.0, 1.0 ], 
                        normal: [0.0, 0.0, 1.0], 
                        color: [1.0, 0.0, 0.0],
                        uv: [0.0, 0.0]
                    },
                    Vertex { 
                        position: [  0.5, -0.5, 1.0, 1.0 ], 
                        normal: [0.0, 0.0, 1.0], 
                        color: [0.0, 1.0, 0.0] ,
                        uv: [0.0, 1.0]
                    },
                    Vertex { 
                        position: [  0.0,  0.5, 1.0, 1.0 ], 
                        normal: [0.0, 0.0, 1.0], 
                        color: [0.0, 0.0, 1.0],
                        uv: [0.5, 0.0]
                    },
                ]
            ) 
        };
    }
}

impl<'a> Renderable for Triangle {
    fn render(&mut self, engine: &mut Graphics) {
        self.mesh.render(engine);
    }
}

impl<'a> Deref for Triangle {
    type Target = Mesh;
    fn deref(&self) -> &Mesh {
        return &self.mesh
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    transformation: Matrix4<f32>,
    index: Option<gfx::Slice<back::Resources>>,
    data: Option<pipe::Data<back::Resources>>,
    pub texture: Option<Texture>
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        return Mesh{
            vertices: vertices,
            transformation: Matrix4::from_scaled_axis(Vector3::new(0., 0.,0.)),
            texture: Option::Some(Texture::default()),
            index: Option::None,
            data: Option::None
        }
    }

    pub fn new_with_texture(vertices: Vec<Vertex>, texture: Texture) -> Self {
        return Mesh{
            vertices: vertices,
            transformation: Matrix4::from_scaled_axis(Vector3::new(0., 0.,0.)),
            texture: Option::Some(texture),
            index: Option::None,
            data: Option::None
        }
    }

    pub fn set_rotation(&mut self, angles: Vector3<f32>) {
        let rotation = Rotation3::new(angles);
        self.transformation = rotation.to_homogeneous() * self.transformation;
    }

    fn bind(&mut self, engine: &mut Graphics) {
        let light_buffer = engine.factory.create_constant_buffer(1);
        let camera_buffer = engine.factory.create_constant_buffer(1);
        let transform_buffer = engine.factory.create_constant_buffer(1);
        let (vertex_buffer, index) = engine.factory.create_vertex_buffer_with_slice(self.vertices.as_slice(), ());
        if self.texture.is_none() {
            return;
        }
        let mut texture = self.texture.clone().unwrap();
        if texture.resource.is_none() {
            let result = engine.bind(texture);
            if result.is_err() {
                return;
            }

            texture = result.unwrap();
        }

        if texture.resource.is_none() {
            return;
        }
        let sampler = engine.factory.create_sampler_linear();
        let data = pipe::Data {
            vbuf: vertex_buffer,
            light: light_buffer,
            camera: camera_buffer,
            transformation: transform_buffer,
            texture: (texture.resource.unwrap(), sampler),
            out: engine.color.clone(),
            depth: engine.depth.clone()
        };

        self.index = Option::Some(index);
        self.data = Option::Some(data);
    }
}

impl <'a> Renderable for Mesh {
    fn render(&mut self, engine: &mut Graphics) {
        if self.data == Option::None || self.index == Option::None {
            self.bind(engine);
        }
    
        let camera_buffer = Camera{ 
            transform: engine.camera.get_view(),
            projection: engine.camera.get_projection()
        };
        let transform = Transform{matrix: self.transformation.into() };
        let data = self.data.as_ref().unwrap();
        let index = self.index.as_ref().unwrap();
        engine.encoder.update_buffer(&data.light, &engine.lights, 0).unwrap();
        engine.encoder.update_buffer(&data.transformation, &[transform], 0).unwrap();
        engine.encoder.update_buffer(&data.camera, &[camera_buffer], 0).unwrap();
        engine.encoder.draw(index, &engine.pso, data);
    }
}