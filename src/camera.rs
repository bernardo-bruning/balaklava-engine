extern crate nalgebra as na;
use na::{Matrix4, Point3, Orthographic3, Vector3, Translation3};

pub struct Orthographic {
    projection: Matrix4<f32>,
    view: Matrix4<f32>
}

impl Orthographic {
    pub fn new(position: Point3<f32>, direction: Point3<f32>) -> Self {
        let projection = Orthographic3::new(-1., 1., -1., 1., 0., 100.).to_homogeneous();
        let view = Matrix4::look_at_rh(&position, &direction, &Vector3::new(0., 1., 0.));

        return Self{
            projection: projection,
            view: view
        }
    }

    pub fn get_projection(&self) -> [[f32; 4]; 4] {
        return self.projection.transpose().into();
    }

    pub fn get_view(&self) -> [[f32;4]; 4] {
        let matrix = self.view;
        return matrix.into();
    }

    pub fn translate(&mut self, x:f32, y:f32, z:f32) {
        let translation = Translation3::new(x, y, z).to_homogeneous();
        self.view = translation*self.view;
    }
}

impl Default for Orthographic {
    fn default() -> Self {
        return Orthographic::new(Point3::new(0., 0., 1.), Point3::new(0., 0., 0.));
    }
}