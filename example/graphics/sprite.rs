extern crate balaklava;
use balaklava::{Application, lauch_gl};
use balaklava_gl::GlDevice;
use balaklava_gpu::{Device};
use balaklava_g2d::Sprite;
use std::path::PathBuf;

struct Game <D: Device> {
    sprite: Sprite<D>
}

impl<D: Device> Application<D> for Game<D> {
    fn new(_: &mut D) -> Self {
        let sprite = Sprite::from(PathBuf::from("example/graphics/texture.png"));
        Game {
            sprite
        }
    }
    
    fn render(&mut self, device: &mut D) {
        self.sprite.render(device);
    }
}

fn main() {
    lauch_gl::<Game<GlDevice>>();
}