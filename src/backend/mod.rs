pub mod gfx;
use crate::graphics::{ShaderProgram};
use std::marker::PhantomData;

pub struct Handle<T> {
    identifier: u64,
    type_marker: PhantomData<T>
}

pub trait Binder<T> {
    fn bind(&mut self, resource: T) -> Handle<T>;
}

pub trait Graphic : 
    Binder<ShaderProgram> {}

pub trait Backend {
    fn graphic(&mut self) -> &mut dyn Graphic;
}