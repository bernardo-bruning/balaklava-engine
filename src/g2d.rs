use std::path::PathBuf;
use balaklava_gpu::Vector;
use balaklava_gpu::Device;

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
            let content = std::fs::read(self.path.as_path()).unwrap();
            let cursor = std::io::Cursor::new(content);
            let texture = device.create_texture(cursor);
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