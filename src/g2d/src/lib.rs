use std::path::PathBuf;
use balaklava_gpu::Device;

pub struct Sprite<D: Device> {
    path: PathBuf,
    texture: Option<D::Texture>
}

impl <D: Device> From<PathBuf> for Sprite<D> {
    fn from(path: PathBuf) -> Self{
        Sprite::<D>{
            path: path,
            texture: Option::None
        }
    }
}