use crate::graphics::{Bindable, Texture};
pub mod gfx;

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

pub trait Graphics<TR> : Bindable<Texture<TR>> {}

pub trait Backend <TR, G: Graphics<TR>> {
    fn graphics() -> G;
}