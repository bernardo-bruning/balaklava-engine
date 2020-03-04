use crate::geometry::Mesh;
use tobj::{Model, Material};
use std::path::Path;
use std::fs::File;
use std::error::Error;

pub struct Obj {
    models: Vec<Model>,
    materials: Vec<Material>
}

impl Obj {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let result = tobj::load_obj(path.as_ref());
        if result.is_err() {
            let error = result.unwrap_err();
            return Result::Err(String::from(error.description()));
        }

        let (models, materials) = result.unwrap();

        return Result::Ok(Obj {
            models,
            materials
        })
    }
}

impl From<File> for Obj {
    fn from(file: File) -> Self{
        unimplemented!();
    }
}

impl <'a> Into<Mesh<'a>> for Obj {
    fn into(self) -> Mesh<'a> {
        unimplemented!();
    }
}