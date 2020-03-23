use crate::Application;

pub fn launch<A>(mut app: A) where A: Application {
    app.create();
    loop {
        app.render();
    }
}