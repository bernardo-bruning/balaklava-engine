use nalgebra::Vector3;

pub struct ShaderProgram {
    pub vertex_shader: Vec<u8>,
    pub fragment_shader: Vec<u8>,
    pub vertices: Vec<Vector3<f32>>
}

impl ShaderProgram {
    pub fn new(vertex_shader: Vec<u8>, fragment_shader: Vec<u8>) -> Self {
        ShaderProgram {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            vertices: Vec::new()
        }
    }
}

pub trait Bindable<T> {
    fn bind(&mut self, bindable: &mut T) -> bool;
}