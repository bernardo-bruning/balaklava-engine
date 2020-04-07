extern crate gfx;
use balaklava_gpu::Vector;


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

pub fn as_vertex(vertices: Vec<Vector>) -> Vec<Vertex> {
    let vec_own = vertices.to_owned();
    let vertex = vec_own.iter().map(|vertice: &Vector| 
        Vertex{ position: [vertice[0], vertice[1], vertice[2], 1.] }
    ).collect();
    vertex
}