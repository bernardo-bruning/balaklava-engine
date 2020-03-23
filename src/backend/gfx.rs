use crate::Application;


pub fn launch<A>() where A: Application {
    let mut app = A::new();
    app.create();
    loop {
        app.render();
    }
}