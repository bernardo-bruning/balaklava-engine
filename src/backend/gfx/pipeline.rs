extern crate gfx;
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