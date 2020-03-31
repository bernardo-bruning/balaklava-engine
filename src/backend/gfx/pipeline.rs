extern crate gfx;
use nalgebra::Vector3;
use gfx::macros;


gfx_defines!{
    vertex Vertex {
        position: [f32; 4] = "vertex_position",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<gfx::format::Srgba8> = "target",
        depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub fn as_vertex(vertices: Vec<Vector3<f32>>) -> Vec<Vertex> {
    let vec_own = vertices.to_owned();
    vec_own.iter().map(|vertice: &Vector3<f32>| 
        Vertex{ position: [vertice[0], vertice[1], vertice[2], 0.] }
    ).collect()
}