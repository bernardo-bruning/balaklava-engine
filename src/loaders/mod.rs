use crate::geometry::Mesh;
use crate::core::Vertex;
use tobj::{Model, Material};
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::iter::Map;

#[derive(Debug)]
pub struct Obj {
    models: Vec<Model>,
    materials: Vec<Material>
}

impl Obj {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file_name = path.as_ref();
        let result = tobj::load_obj(file_name);
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

impl <'a> Into<Result<Mesh, String>> for Obj {
    fn into(self) -> Result<Mesh, String> {
        let result_model = self.models.first();
        if result_model.is_none() {
            return Result::Err("Not contains model in OBJ file.".to_string());
        }
        let model: &tobj::Model = result_model.unwrap();
        let positions = model.mesh.positions.clone();
        let mut vertices = Vec::new();
        for f in 0..model.mesh.indices.len()/3 {
            let v1 = model.mesh.indices[3*f];
            let v2 = model.mesh.indices[3*f + 1];
            let v3 = model.mesh.indices[3*f + 2];

            for vi in vec![v1, v2, v3] {
                let v = (vi as usize);
                let x = positions[3*v];
                let y = positions[3*v+1];
                let z = positions[3*v+2];
                vertices.push(Vertex {
                    position: [ x, y, z, 1.0 ], 
                    normal: [0.0, 0.0, 0.0], 
                    color: [0.0, 0.0, 0.0],
                    uv: [0.0, 0.0]
                })
            }
        }
        
        return Result::Ok(Mesh::new(vertices));
    }
}