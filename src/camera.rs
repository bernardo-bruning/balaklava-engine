extern crate nalgebra as na;
use na::{Matrix4, Point3, Orthographic3, Vector3, Translation3};

fn DefaultOrtographic3() -> Matrix4<f32> {
    return Orthographic3::new(-10., 10., -10., 10., 0., 100.).to_homogeneous();
}

pub struct Orthographic {
    projection: Matrix4<f32>,
    view: Matrix4<f32>
}

impl Orthographic {
    pub fn new(position: Point3<f32>, direction: Point3<f32>) -> Self {
        let projection = DefaultOrtographic3();
        let view = Matrix4::look_at_rh(&position, &direction, &Vector3::new(0., 1., 0.));

        return Self{
            projection: projection,
            view: view
        }
    }

    pub fn get_view(&self) -> [[f32;4]; 4] {
        return (self.projection * self.view).into();
    }

    pub fn translate(&mut self, x:f32, y:f32, z:f32) {
        let translation = Translation3::new(x, y, z).to_homogeneous();
        self.view = self.view*translation;
    }
}

impl Default for Orthographic {
    fn default() -> Self {
        return Orthographic::new(Point3::new(0., 0., 1.), Point3::new(0., 0., 0.));
    }
}