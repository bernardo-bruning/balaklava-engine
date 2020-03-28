extern crate balaklava;
use balaklava::DummyApplication;
use balaklava::backend::gfx::{Backend, Config};

fn main() {
    let config = Config::default()
        .with_fullscreen(true);
    let mut backend = Backend::new(config);
    let mut app = DummyApplication::default();
    backend.launch(&mut app)
}