extern crate gfx;
extern crate gfx_device_gl as back;
extern crate nalgebra as na;
use crate::core::*;
use na::{Matrix4,Vector3,Rotation3};
use gfx::traits::FactoryExt;

pub struct Mesh<'a> {
    vertices: &'a[Vertex],
    transformation: Matrix4<f32>,
    index: Option<gfx::Slice<back::Resources>>,
    data: Option<pipe::Data<back::Resources>>
}

impl <'a> Mesh<'a> {
    pub fn new(vertices: &'a[Vertex]) -> Self {
        return Mesh{
            vertices: vertices,
            transformation: Matrix4::from_scaled_axis(Vector3::new(0., 0.,0.)),
            index: Option::None,
            data: Option::None
        }
    }

    fn set_rotation(&mut self, angles: Vector3<f32>) {
        let rotation = Rotation3::new(angles);
        self.transformation = rotation.to_homogeneous() * self.transformation;
    }

    fn bind(&mut self, engine: &mut Engine) {
        let light_buffer = engine.factory.create_constant_buffer(1);
        let camera_buffer = engine.factory.create_constant_buffer(1);
        let transform_buffer = engine.factory.create_constant_buffer(1);
        let (vertex_buffer, index) = engine.factory.create_vertex_buffer_with_slice(self.vertices, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            light: light_buffer,
            camera: camera_buffer,
            transformation: transform_buffer,
            out: engine.color.clone(),
        };

        self.index = Option::Some(index);
        self.data = Option::Some(data);
    }

    pub fn render(&mut self, engine: &mut Engine) {
        if self.data == Option::None || self.index == Option::None {
            self.bind(engine);
        }
    
        let camera_buffer = Camera{ transform: engine.camera.get_view() };
        let transform = Transform{matrix: self.transformation.into() };
        let data = self.data.as_ref().unwrap();
        let index = self.index.as_ref().unwrap();
        engine.encoder.update_buffer(&data.light, &engine.lights, 0).unwrap();
        engine.encoder.update_buffer(&data.transformation, &[transform], 0).unwrap();
        engine.encoder.update_buffer(&data.camera, &[camera_buffer], 0).unwrap();
        engine.encoder.draw(index, &engine.pso, data);
    }
}