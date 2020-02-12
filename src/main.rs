extern crate glutin;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Teste Aplicativo");
    let _window = window_builder
        .build(&event_loop)
        .unwrap();

    event_loop.run(|event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent{ event, .. } => 
                if event == glutin::event::WindowEvent::CloseRequested {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                },
            _ => ()
        }
    });
}