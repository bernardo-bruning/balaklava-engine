use crate::geometry::Mesh;
use crate::core::Vertex;
use tobj::{Model, Material};
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::iter::Map;

mod test;

#[derive(Debug)]
pub struct Obj {
    models: Vec<Model>,
    materials: Vec<Material>
}

fn load_vector_n(plan_vector: &Vec<f32>, v: usize, n: usize) -> Vec<f32>{
    let mut result = Vec::new();

    for i in 0..n {
        if plan_vector.len() > n*v+i {
            result.push(plan_vector[n*v+i]);
        }
    }

    return result;
}

fn load_vector(plan_vector: &Vec<f32>, v: usize) -> [f32; 3]{
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    if plan_vector.len() > 3*v {
        x = plan_vector[3*v];
    }
    if plan_vector.len() > 3*v+1 {
        y = plan_vector[3*v+1];
    }
    if plan_vector.len() > 3*v+2 {
        z = plan_vector[3*v+2];
    }

    return [x, y, z];
}

impl Obj {
    pub fn new(models: Vec<Model>, materials: Vec<Material>) -> Obj {
        Obj {
            models,
            materials
        }
    }

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
        let mut vertices = Vec::new();
        for f in 0..model.mesh.indices.len()/3 {
            let v1 = model.mesh.indices[3*f];
            let v2 = model.mesh.indices[3*f + 1];
            let v3 = model.mesh.indices[3*f + 2];

            for v in vec![v1, v2, v3] {
                let position = load_vector_n(&model.mesh.positions, v as usize, 3);
                let normals = load_vector_n(&model.mesh.normals, v as usize, 3);
                let uv = load_vector_n(&model.mesh.texcoords, v as usize, 2);
                
                vertices.push(Vertex {
                    position: [ position[0], position[1], position[2], 1.0 ], 
                    normal: [normals[0], normals[1], normals[2]], 
                    color: [0.0, 0.0, 0.0],
                    uv: [uv[0], uv[1]]
                })
            }
        }
        
        return Result::Ok(Mesh::new(vertices));
    }
}