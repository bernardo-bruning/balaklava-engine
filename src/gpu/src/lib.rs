extern crate nalgebra;
use nalgebra::{Vector3, Matrix4};
use std::io::{BufRead, Seek};
use std::ops::Mul;
use nalgebra::Orthographic3;

pub type Vector = Vector3<f32>;

pub struct Camera {
    matrix: Orthographic3<f32>
}

impl Mul<Transform> for Camera {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            matrix: rhs.matrix * self.matrix.to_homogeneous()
        }
    }
}

impl Into<[[f32; 4]; 4]> for Camera {
    fn into(self) -> [[f32;4]; 4] {
        let camera = self.matrix.to_homogeneous();
        camera.into()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            matrix: Orthographic3::new(1., 1., 1., 1., 0.1, 100.0)
        }
    }
}

pub struct Transform {
    matrix: Matrix4<f32>
}

impl Into<[[f32; 4]; 4]> for Transform {
    fn into(self) -> [[f32; 4]; 4] {
        self.matrix.into()
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            matrix: Matrix4::<f32>::identity()
        }
    }
}

pub trait Device {
    type Program;
    type Buffer;
    type Texture;
    fn create_program(&mut self, vertex_shader: Vec<u8>, pixel_shader: Vec<u8>) -> Self::Program;
    fn create_vertex_buffer(&mut self, program: &mut Self::Program, vertices: Vec<Vector>) -> Self::Buffer;
    fn create_texture<R: BufRead+Seek>(&mut self, reader: R) -> Self::Texture;
    fn render_program(&mut self, program: &Self::Program, buffer: &Self::Buffer, transform: Option<Transform>, texture: Option<&Self::Texture>);
    fn flush(&mut self);
}