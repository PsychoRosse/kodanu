use wgpu::{
    BufferAddress as WgpuBufferAddress, VertexAttribute as WgpuVertexAttribute,
    VertexBufferLayout as WgpuVertexBufferLayout, VertexStepMode as WgpuVertexStepMode,
};

use {
    bytemuck::{Pod, Zeroable},
    kodanu_macros::Getters,
};

#[repr(C)]
#[derive(Getters, Debug, Clone, Copy, Pod, Zeroable, PartialEq, PartialOrd)]
pub struct Vertex {
    #[getter(copy)]
    position: [f32; 3],
    #[getter(copy)]
    normal: [f32; 3],
    #[getter(copy)]
    uv: [f32; 2],
}

impl Vertex {
    pub const ATTRIBUTES: [WgpuVertexAttribute; 3] = [
        WgpuVertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        },
        WgpuVertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 12,
            shader_location: 1,
        },
        WgpuVertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 24,
            shader_location: 2,
        },
    ];
}

impl Vertex {
    pub const fn new(position: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}

impl Vertex {
    pub const fn layout<'v>() -> WgpuVertexBufferLayout<'v> {
        WgpuVertexBufferLayout {
            array_stride: size_of::<Vertex>() as WgpuBufferAddress,
            step_mode: WgpuVertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
