use std::path::PathBuf;
use balaklava_math::Vector;
use balaklava_math::Transform;
use balaklava_gpu::Device;
use std::io::Cursor;

#[derive(Debug, Clone)]
struct Rectangle {
    a: Vector,
    b: Vector,
    c: Vector,
    d: Vector
}

impl From<Vector> for Rectangle {
    fn from(vector: Vector) -> Self {
        Self {
            a: Vector::new(0., 0., 0.),
            b: Vector::new(vector[0], 0., 0.),
            c: Vector::new(vector[0], vector[1], 0.),
            d: Vector::new(0., vector[1], 0.)
        }
    }
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
pub struct Texture<D: Device> {
    image: image::RgbaImage,
    transform: Transform,
    instance: Option<D::Texture>
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
            instance: Option::None
        }
    }

    fn dimensions(&self) -> Vector {
        let (x, y) = self.image.dimensions();
        return Vector::new(x as f32, y as f32, 0.);
    }

    fn bind(&mut self, device: &mut D) {
        if self.instance.is_some() {
            return
        }

        let instance = device.create_texture(self.image.clone().into_raw(), self.dimensions());
        self.instance = Option::Some(instance);
    }
}

impl <D:Device> Into<Cursor<Vec<u8>>> for Texture<D> {
    fn into(self) -> Cursor<Vec<u8>> {
        return std::io::Cursor::new(self.image.into_vec());
    }
}

pub struct Sprite<D: Device> {
    texture: Texture<D>,
    pub transform: Transform,
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

        self.texture.bind(device);

        if self.buffer.is_none() {
            let program = self.program.as_mut().unwrap();
            let dimension = Rectangle::default().into();
            let buffer = device.create_vertex_buffer(program, dimension);
            self.buffer = Option::Some(buffer);
        }

        device.render_program(
            self.program.as_ref().unwrap(), 
            self.buffer.as_ref().unwrap(), 
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
            buffer: Option::None
        }
    }
}