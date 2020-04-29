use std::path::PathBuf;
use balaklava_math::{Vector, Transform, Rectangle};
use balaklava_gpu::Device;
use std::io::Cursor;


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

    fn bind(&mut self, device: &mut D, program: &mut D::Program) {
        if self.instance.is_none() {
            let instance = device.create_texture(self.image.clone().into_raw(), self.dimensions());
            self.instance = Option::Some(instance);
        }

        if self.buffer.is_none() {
            let dimension = Rectangle::default().into();
            let buffer = device.create_vertex_buffer(program, dimension);
            self.buffer = Option::Some(buffer);
        }
    }
}

impl <D:Device> Into<Cursor<Vec<u8>>> for Texture<D> {
    fn into(self) -> Cursor<Vec<u8>> {
        return std::io::Cursor::new(self.image.into_vec());
    }
}

pub struct Sprite<D: Device> {
    texture: Texture<D>,
    program: Option<D::Program>,
    pub transform: Transform
}

impl <D: Device> Sprite<D> {
    pub fn render(&mut self, device: &mut D) {
        if self.program.is_none() {
            let vertex_shader = include_bytes!("sharder/shader_150.glslv");
            let pixel_shader = include_bytes!("sharder/shader_150.glslf");
            let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec());
            self.program = Option::Some(program);
        }

        self.texture.bind(device, self.program.as_mut().unwrap());

        device.render_program(
            self.program.as_ref().unwrap(), 
            self.texture.buffer.as_ref().unwrap(), 
            Option::Some(&self.transform*&self.texture.transform),
            self.texture.instance.as_ref());
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
            texture: Texture::new(&path),
            transform: Transform::default(),
            program: Option::None,
        }
    }
}