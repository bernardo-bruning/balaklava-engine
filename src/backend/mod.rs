use crate::graphics::{ShaderProgram};
pub mod gfx;

pub enum Handle<T> {
    None,
    Some(i32, T)
}

pub trait Bindable<T> {
    fn bind(&mut self, resource: T) -> Handle<T>;
}

pub trait Graphic : Bindable<ShaderProgram> {}

pub trait Backend {
    fn graphic(&mut self) -> &mut dyn Graphic;
}