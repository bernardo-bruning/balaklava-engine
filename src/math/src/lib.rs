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

impl Transform {
    pub fn translate(&mut self, vector: Vector){
        let translation = Matrix4::new_translation(&vector);
        self.matrix = self.matrix*translation;
    }
}

impl From<Vector> for Transform {
    fn from(vector: Vector) -> Self {
        let matrix = Matrix4::<f32>::new_nonuniform_scaling(&vector);
        Transform {
            matrix: matrix
        }
    }
}

impl Mul<&Transform> for &Transform {
    type Output = Transform;
    fn mul(self, rhs: &Transform) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix
        }
    }
}

impl Mul<&Vector> for &Transform {
    type Output = Transform;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let matrix = Matrix4::<f32>::new_nonuniform_scaling(&rhs).transpose();
        Transform {
            matrix: self.matrix * matrix
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

#[derive(Debug, Clone)]
pub struct Rectangle {
    a: Vector,
    b: Vector
}

impl From<Vector> for Rectangle {
    fn from(vector: Vector) -> Self {
        Self {
            a: Vector::new(0., 0., 0.),
            b: vector
        }
    }
}

impl From<(Vector, Vector)> for Rectangle {
    fn from(vectors: (Vector, Vector)) -> Self {
        let (a, b) = vectors;
        Self {
            a: a,
            b: b
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            a: Vector::new(0.0, 0.0, 0.0),
            b: Vector::new(1.0, 1.0, 0.0),
        }
    }
}

impl Into<Vector> for Rectangle {
    fn into(self) -> Vector {
        return self.b - self.a
    }
}

impl Into<Vec<Vector>> for Rectangle {
    fn into(self) -> Vec<Vector> {
        let mut rect = Vec::new();
        rect.push(self.a);
        rect.push(Vector::new(self.a[0], self.b[1], 0.));
        rect.push(self.b);
        rect.push(self.a);
        rect.push(Vector::new(self.b[0], self.a[1], 0.));
        rect.push(self.b);
        return rect
    }
}
