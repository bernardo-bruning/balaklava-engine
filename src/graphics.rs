#[derive(Debug, Clone)]
pub struct Texture<R> {
    data: Vec<u8>,
    width: u16,
    height: u16,
    pub resource: R
}

pub struct ShaderProgram {
    pub vertex_shader: String,
    pub fragment_shader: String
}

impl ShaderProgram {
    fn new(vertex_shader: String, fragment_shader: String) -> Self {
        ShaderProgram {
            vertex_shader,
            fragment_shader
        }
    }
}

pub trait Bindable<T> {
    fn bind(&mut self, bindable: &mut T) -> bool;
}