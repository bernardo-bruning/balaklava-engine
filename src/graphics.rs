use nalgebra::Vector3;

pub struct ShaderProgram<'a> {
    pub vertex_shader: &'a[u8],
    pub fragment_shader: &'a[u8],
    pub vertices: Vec<Vector3<f32>>
}

impl <'a> ShaderProgram<'a> {
    fn new(vertex_shader: &'a[u8], fragment_shader: &'a[u8]) -> Self {
        ShaderProgram {
            vertex_shader,
            fragment_shader,
            vertices: Vec::new()
        }
    }
}

pub trait Bindable<T> {
    fn bind(&mut self, bindable: &mut T) -> bool;
}