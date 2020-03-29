extern crate balaklava;
use balaklava::backend::gfx::Backend as GfxBackend;
use balaklava::backend::{Backend};
use balaklava::Application;

struct SquareApplication {
}

impl Application for SquareApplication {
    fn run(&mut self, backend: &mut dyn Backend){}
}

fn main() {
    let mut backend = GfxBackend::default();
    backend.launch(SquareApplication{});
}