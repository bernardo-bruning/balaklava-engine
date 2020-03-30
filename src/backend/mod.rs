use crate::graphics::{Bindable, ShaderProgram};
pub mod gfx;

pub trait Graphic : Bindable<ShaderProgram> {}

pub trait Backend {
    fn graphic(&mut self) -> &mut dyn Graphic;
}