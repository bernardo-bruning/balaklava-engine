#[cfg(test)]
use crate::geometry as g;
use tobj::{Model, Mesh};
use crate::core::Vertex;
use crate::loaders::Obj;

#[test]
fn test_obj_into_mesh() {
    let positions = vec![
        0., 0., 0.,
        0., 1., 0.,
        1., 0., 0.,
        1., 1., 0.
    ];

    let normals: Vec<f32> = vec![];

    let indices = vec![
        0, 1, 2,
        1, 2, 3
    ];

    let models = vec![
        Model::new(Mesh::new(
            positions, 
            vec![], 
            vec![], 
            indices, 
            Option::None
        ), "plane".to_string())
    ];

    let materials = vec![];
    let obj = Obj::new(models, materials);
    let mesh_result: Result<g::Mesh, String> = obj.into();
    let mesh: g::Mesh = mesh_result.unwrap();
    assert_eq!(vec![
        Vertex {
            position: [ 0., 0., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [ 0., 1., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [ 1., 0., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [ 0., 1., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [ 1., 0., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [ 1., 1., 0., 1.0 ], 
            normal: [0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        },
    ], mesh.vertices);
}