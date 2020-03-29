extern crate balaklava;
use balaklava::backend::gfx::{Backend, Config};
use balaklava::DummyApplication;

fn main() {
    let config = Config::default()
        .with_title("Application Example".to_string())
        .with_dimensions(200, 200);

    let mut backend = Backend::new(config);
    let mut application = DummyApplication::default();
    backend.launch(application);
}