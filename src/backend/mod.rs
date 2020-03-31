pub mod gfx;
use crate::graphics::{ShaderProgram};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Handle<T> {
    identifier: u64,
    type_marker: PhantomData<T>
}

pub trait Binder<T> {
    fn bind(&mut self, resource: T) -> Handle<T>;
}

pub trait Render<T> {
    fn render(&mut self, renderable: &Handle<T>);
}

pub trait Graphic : 
    Binder<ShaderProgram> 
    + Render<ShaderProgram> {}

pub trait Backend {
    fn graphic(&mut self) -> &mut dyn Graphic;
}