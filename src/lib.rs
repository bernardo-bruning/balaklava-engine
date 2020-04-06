extern crate nalgebra as na;
extern crate log;
extern crate env_logger;

mod core;
mod geometry;
mod camera;
mod loaders;

// impl Application for Example {

//     fn create(&mut self) {
//         env_logger::init();
//         info!("Initialize program");
//         let builder = core::Builder::default()
//             .with_name("Learn GFX".to_string())
//             .with_light(Light {
//                 position: [0.5, 0.0, 0.07, 1.0],
//                 color: [1.0, 1.0, 1.0]
//             });

//         let obj_result = Obj::open("./penguin.obj");
//         if obj_result.is_err() {
//             let error: String = obj_result.unwrap_err();
//             panic!("One error ocurred: {}", error);
//         }

//         let obj = obj_result.unwrap();
//         let r: Result<u32, String> = Result::Err("test".to_string());
//         let model_result: Result<geometry::Mesh, String> = obj.into();
//         if model_result.is_err() {
//             panic!("One error ocurred when convert Obj into Mesh: {}", model_result.unwrap_err());
//         }

//         let mut model = model_result.unwrap();
//         model.texture = Texture::load("texture.png".to_string()).ok();
        
//         let mut engine = std::boxed::Box::from(builder.build());

//         let mut running = true;
//         info!("Initialize render");
//         while running {
//             model.set_rotation(na::Vector3::new(0., 0.01, 0.));
//             //engine.camera.translate(0., 0., -0.01);
//             engine.poll_event(|event| match event {
//                 events::Event::Closed => running = false,
//                 events::Event::KeyInput{ input } => 
//                     match input.virtual_key {
//                         //Option::Some(events::KeyCodes::W) => engine.camera.translate(1., 0., 0.),
//                         //Option::Some(events::KeyCodes::W) => println!("teste"),
//                         _ => ()
//                     },
//                 _ => ()
//             });

//             engine.clear();
//             model.render(&mut engine);
//             engine.update();
//         }
//         info!("Close program");
//     }

//     fn render(&mut self) {

//     }
// }
