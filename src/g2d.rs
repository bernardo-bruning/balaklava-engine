use std::path::PathBuf;
use balaklava_gpu::Vector;
use balaklava_gpu::Device;
use std::io::Cursor;

#[derive(Debug, Clone)]
struct Rectangle {
    a: Vector,
    b: Vector,
    c: Vector,
    d: Vector
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            a: Vector::new(0.0, 0.0, 0.0),
            b: Vector::new(1.0, 0.0, 0.0),
            c: Vector::new(1.0, 1.0, 0.0),
            d: Vector::new(0.0, 1.0, 0.0)
        }
    }
}

impl Into<Vec<Vector>> for Rectangle {
    fn into(self) -> Vec<Vector> {
        let mut rect = Vec::new();
        rect.push(self.a);
        rect.push(self.b);
        rect.push(self.c);
        rect.push(self.d);
        rect.push(self.c);
        rect.push(self.a);
        return rect
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    image: image::RgbaImage
}

impl Texture {
    fn new(path: &PathBuf) -> Self{
        let content = std::fs::read(path.as_path()).unwrap();
        let cursor = Cursor::new(content);
        let image = image::load(cursor, image::ImageFormat::Png)
            .unwrap().to_rgba();

        Texture {
            image
        }
    }

    fn dimensions(&self) -> Vector {
        let (x, y) = self.image.dimensions();
        return Vector::new(x as f32, y as f32, 0.);
    }
}

impl Into<Cursor<Vec<u8>>> for Texture {
    fn into(self) -> Cursor<Vec<u8>> {
        return std::io::Cursor::new(self.image.into_vec());
    }
}

pub struct Sprite<D: Device> {
    path: PathBuf,
    texture: Option<D::Texture>,
    program: Option<D::Program>,
    buffer: Option<D::Buffer>
}

impl <D: Device> Sprite<D> {
    pub fn render(&mut self, device: &mut D) {
        if self.program.is_none() {
            let vertex_shader = include_bytes!("sharder/shader_150.glslv");
            let pixel_shader = include_bytes!("sharder/shader_150.glslf");
            let program = device.create_program(vertex_shader.to_vec(), pixel_shader.to_vec());
            self.program = Option::Some(program);
        }

        if self.texture.is_none() {
            let texture = Texture::new(&self.path);
            let texture = device.create_texture(texture.image.clone().into_raw(), texture.dimensions());
            self.texture = Option::Some(texture);
        }

        if self.buffer.is_none() {
            let program = self.program.as_mut().unwrap();
            let buffer = device.create_vertex_buffer(program, Rectangle::default().into());
            self.buffer = Option::Some(buffer);
        }

        device.render_program(
            self.program.as_ref().unwrap(), 
            self.buffer.as_ref().unwrap(), 
            Option::None,
            self.texture.as_ref());
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
            path: path,
            texture: Option::None,
            program: Option::None,
            buffer: Option::None
        }
    }
}