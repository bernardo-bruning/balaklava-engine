extern crate nalgebra;
use nalgebra::{Vector3, Matrix4};
use std::ops::Mul;
use nalgebra::Orthographic3;

pub type Vector = Vector3<f32>;

#[derive(Debug, Clone)]
pub struct Camera {
    matrix: Orthographic3<f32>
}

impl Mul<Transform> for Camera {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            matrix: self.matrix.to_homogeneous() * rhs.matrix
        }
    }
}

impl Into<[[f32; 4]; 4]> for Camera {
    fn into(self) -> [[f32;4]; 4] {
        let camera = self.matrix.to_homogeneous();
        camera.into()
    }
}

impl From<(f32, f32)> for Camera {
    fn from(dimension: (f32, f32)) -> Self {
        let (width, height) = dimension;
        Camera {
            matrix: Orthographic3::new(0., width, 0., height, -0.1, 1000.0)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            matrix: Orthographic3::new(0., 1., 0., 1., -0.1, 1000.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    matrix: Matrix4<f32>
}

impl From<Vector> for Transform {
    fn from(vector: Vector) -> Self {
        let matrix = Matrix4::<f32>::new_nonuniform_scaling(&vector);
        Transform {
            matrix: matrix
        }
    }
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
    fn create_texture(&mut self, data: Vec<u8>, dimensions: Vector) -> Self::Texture;
    fn render_program(&mut self, program: &Self::Program, buffer: &Self::Buffer, transform: Option<Transform>, texture: Option<&Self::Texture>);
    fn flush(&mut self);
}