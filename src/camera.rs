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

    pub fn get_view(&self) -> [[f32;4]; 4] {
        let matrix = self.projection*self.view;
        println!("{}", matrix);
        let array: [[f32; 4]; 4] = [
            [matrix[0], matrix[4], matrix[8], matrix[12]],
            [matrix[1], matrix[5], matrix[9], matrix[13]],
            [matrix[2], matrix[6], matrix[10], matrix[14]],
            [matrix[3], matrix[7], matrix[11], matrix[15]],
        ];
        return array;
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