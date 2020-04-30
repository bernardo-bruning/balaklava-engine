extern crate balaklava;
use balaklava::{Application, lauch_gl};
use balaklava::math::Rectangle;
use balaklava::gpu::Device;
use balaklava::Vector;
use balaklava::g2d::{Sprite, TextureRegion, Texture};
use balaklava_gl::GlDevice;

struct Game <D: Device> {
    sprite: Sprite<D>
}

impl<D: Device> Application<D> for Game<D> {
    fn new(_: &mut D) -> Self {
        let texture = Texture::from("example/graphics/texture.png");
        let texture_region = TextureRegion::new(texture, Rectangle::from(Vector::new(0.5, 1., 1.)));
        let mut sprite = Sprite::from(texture_region);
        sprite.transform.translate(Vector::new(115., 150., 0.));
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