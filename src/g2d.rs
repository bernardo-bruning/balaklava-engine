use std::path::PathBuf;
use balaklava_math::{Vector, Transform, Rectangle};
use balaklava_gpu::Device;
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Texture<D: Device> {
    image: image::RgbaImage,
    transform: Transform,
    instance: Option<D::Texture>,
    buffer: Option<D::Buffer>
}

impl <D: Device> Texture<D> {
    fn new(path: &PathBuf) -> Self{
        let content = std::fs::read(path.as_path()).unwrap();
        let cursor = Cursor::new(content);
        let image = image::load(cursor, image::ImageFormat::Png)
            .unwrap().to_rgba();
        let (x, y) = image.dimensions();
        Texture {
            image: image,
            transform: Transform::from(Vector::new(x as f32, y as f32, 0.)),
            instance: Option::None,
            buffer: Option::None
        }
    }

    fn dimensions(&self) -> Vector {
        let (x, y) = self.image.dimensions();
        return Vector::new(x as f32, y as f32, 0.);
    }

    fn bind(&mut self, device: &mut D) {
        if self.instance.is_none() {
            let instance = device.create_texture(self.image.clone().into_raw(), self.dimensions());
            self.instance = Option::Some(instance);
        }
    }
}

impl <D:Device> From<&str> for Texture<D> {
    fn from(path: &str) -> Self {
        Texture::new(&PathBuf::from(path))
    }
}

impl <D:Device> Into<Cursor<Vec<u8>>> for Texture<D> {
    fn into(self) -> Cursor<Vec<u8>> {
        return std::io::Cursor::new(self.image.into_vec());
    }
}

pub struct TextureRegion<D: Device> {
    region: Rectangle,
    texture: Rc<RefCell<Texture<D>>>,
    buffer: Option<D::Buffer>
}

impl <D: Device> TextureRegion<D> {
    pub fn new(texture: Texture<D>, rectangle: Rectangle) -> Self {
        return TextureRegion {
            texture: Rc::from(RefCell::from(texture)),
            region: rectangle,
            buffer: Option::None,
        }
    }

    pub fn render(&mut self, device: &mut D, program: &mut D::Program, transform: Transform) {
        if self.buffer.is_none() {
            let dimension = Rectangle::default().into();
            let region = self.region.clone();
            let buffer = device.create_vertex_buffer(program, dimension, Option::Some(region.into()));
            self.buffer = Option::Some(buffer);
        }

        &self.texture.borrow_mut().bind(device);
        let region: Vector = self.region.clone().into();
        device.render_program(
            program, 
            self.buffer.as_ref().unwrap(), 
            Option::Some(&transform*&(&self.texture.borrow().transform*&region)),
            self.texture.borrow().instance.as_ref());
    }

    pub fn set_region(&mut self, x:f32, y:f32, u:f32, v: f32) {
        self.region = Rectangle::from(
            (
                Vector::new(x, y, 0.0), 
                Vector::new(u, v, 0.0)
            )
        )
    }
}

impl <D:Device> From<&PathBuf> for TextureRegion<D> {
    fn from(path: &PathBuf) -> Self{
        let texture = Texture::new(path);
        TextureRegion::new(texture, Rectangle::from(Vector::new(1., 1., 1.)))
    }
}

pub struct Sprite<D: Device> {
    texture: TextureRegion<D>,
    program: Option<D::Program>,
    buffer: Option<D::Buffer>,
    pub transform: Transform
}

impl <D: Device> Sprite<D> {
    fn bind(&mut self, device: &mut D) {
        if self.program.is_none() {
            let vertex_shader = include_bytes!("sharder/shader_150.glslv");
            let pixel_shader = include_bytes!("sharder/shader_150.glslf");
            let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec());
            self.program = Option::Some(program);
        }

        if self.buffer.is_none() {
            let dimension = Rectangle::default().into();
            let region = Rectangle::from(Vector::new(0.5, 0.5, 1.0));
            let buffer = device.create_vertex_buffer(self.program.as_mut().unwrap(), dimension, Option::Some(region.into()));
            self.buffer = Option::Some(buffer);
        }
    }

    pub fn render(&mut self, device: &mut D) {
        self.bind(device);
        self.texture.render(device, self.program.as_mut().unwrap(), self.transform.clone());
    }
}

impl <D: Device> From<&'_ str> for Sprite<D> {
    fn from<'a>(path: &str) -> Self {
        Sprite::from(PathBuf::from(path))
    }
}

impl <D: Device> From<PathBuf> for Sprite<D> {
    fn from(path: PathBuf) -> Self{
        Sprite::<D>{
            texture: TextureRegion::from(&path),
            transform: Transform::default(),
            program: Option::None,
            buffer: Option::None,
        }
    }
}

impl <D: Device> From<TextureRegion<D>> for Sprite<D> {
    fn from(texture_region: TextureRegion<D>) -> Self {
        Sprite::<D>{
            texture: texture_region,
            transform: Transform::default(),
            program: Option::None,
            buffer: Option::None,
        }
    }
}

use std::time::Duration;
use std::collections::BTreeMap;

pub struct Animation<T:Clone> {
    started: Option<Duration>,
    states: BTreeMap<Duration, T>,
    last_state: T
}

impl<T:Clone> Animation<T> {
    pub fn new(initial_state: T) -> Self {
        let states = BTreeMap::new();
        return Animation{ 
            started: Option::None,
            states:  states, 
            last_state: initial_state 
        };
    }

    pub fn insert(&mut self, moment: Duration, state: T) {
        self.states.insert(moment, state);
    }

    pub fn start(&mut self, moment: Duration) {
        self.started = Option::Some(moment);
    }

    pub fn next(&mut self, moment: Duration) -> &T{
        if self.started.is_none() {
            self.start(moment)
        }

        let started = self.started.unwrap();
        let current_opt = self.states.get(&(moment- started));
        self.last_state = current_opt.unwrap_or(&self.last_state).clone();
        return &self.last_state;
    }
}

#[cfg(test)]
mod tests {
    use crate::g2d::Animation;
    use std::time::Duration;

    #[test]
    fn test_animation() {
        let mut animation = Animation::<i32>::new(1);
        animation.insert(Duration::from_secs(1), 2);
        animation.insert(Duration::from_secs(2), 3);
        animation.insert(Duration::from_secs(3), 4);

        animation.start(Duration::from_secs(2));
        assert_eq!(&1, animation.next(Duration::from_secs(2)));
        assert_eq!(&2, animation.next(Duration::from_secs(3)));
        assert_eq!(&3, animation.next(Duration::from_secs(4)));
        assert_eq!(&4, animation.next(Duration::from_secs(5)));
        assert_eq!(&4, animation.next(Duration::from_secs(6)));
    }
}