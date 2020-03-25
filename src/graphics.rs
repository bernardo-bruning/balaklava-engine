#[derive(Debug, Clone)]
pub struct Texture<R> {
    data: Vec<u8>,
    width: u16,
    height: u16,
    pub resource: R
}

pub trait Bindable<T> {
    fn bind(&mut self, bindable: &mut T);
}