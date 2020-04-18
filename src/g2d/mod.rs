use std::path::PathBuf;
use balaklava_gpu::Vector;
use balaklava_gpu::Device;

pub struct Sprite<D: Device> {
    path: PathBuf,
    rect: Vec<Vector>,
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
            let buffer = device.create_vertex_buffer(program, self.rect.clone());
            self.buffer = Option::Some(buffer);
        }

        device.render_program(
            self.program.as_ref().unwrap(), 
            self.buffer.as_ref().unwrap(), 
            Option::None,
            self.texture.as_ref());
    }
}

impl <D: Device> From<PathBuf> for Sprite<D> {
    fn from(path: PathBuf) -> Self{
        let mut rect = Vec::new();
        rect.push(Vector::new(0.0, 0.0, 0.0));
        rect.push(Vector::new(1.0, 0.0, 0.0));
        rect.push(Vector::new(1.0, 1.0, 0.0));
        rect.push(Vector::new(0.0, 1.0, 0.0));
        rect.push(Vector::new(1.0, 1.0, 0.0));
        rect.push(Vector::new(0.0, 0.0, 0.0));

        Sprite::<D>{
            path: path,
            rect: rect, 
            texture: Option::None,
            program: Option::None,
            buffer: Option::None
        }
    }
}