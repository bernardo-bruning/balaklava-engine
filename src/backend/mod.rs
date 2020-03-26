use crate::graphics::{Bindable, Texture};
pub mod gfx;

pub trait Graphics<TR> : Bindable<Texture<TR>> + Bindable<ShaderProgram> {}

pub trait Backend <TR, G: Graphics<TR>> {
    fn graphics() -> G;
}