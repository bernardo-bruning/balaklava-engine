extern crate balaklava;
use balaklava::backend::{Graphics, Backend};
use balaklava::Application;

struct Example {
    created: bool,
    rendered: bool
}

impl Example {
    fn new() -> Self {
        Example{
            created: false,
            rendered: false
        }
    }
}

impl Application for Example {
    fn create<TR, G:Graphics<TR>, B: Backend<TR, G>>(&mut self, backend: &mut B)
    {
        self.created = true;
        println!("Close to check");
    }

    fn render(&mut self)
    {
        self.rendered = true;
    }
}

fn main() {
    let config = balaklava::backend::gfx::Config::default()
        .with_title("Application Example".to_string())
        .with_dimensions(200, 200);

    let mut backend = balaklava::backend::gfx::Backend::new(config);
    let mut application = Example::new();
    backend.launch(&mut application);
    
    assert_eq!(true, application.created);
    assert_eq!(true, application.rendered);
}