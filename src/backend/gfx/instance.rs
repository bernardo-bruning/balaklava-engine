use crate::backend::gfx::pipeline;
use gfx_device_gl as back;
use gfx::handle::Buffer;

pub struct ShaderProgram {
    pub vertex_buffer: Buffer<back::Resources, pipeline::Vertex>,
    pub slice: gfx::Slice<back::Resources>,
    pub pso: gfx::PipelineState<back::Resources, pipeline::pipe::Meta>
}