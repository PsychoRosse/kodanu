use crate::Mesh;

use wgpu::{
    Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice,
    util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt},
};

use {bytemuck::cast_slice, kodanu_macros::Getters};

#[derive(Getters, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GpuMesh {
    #[getter()]
    vertex_buffer: WgpuBuffer,
    #[getter()]
    index_buffer: WgpuBuffer,
    #[getter(copy)]
    index_count: u32,
}

impl GpuMesh {
    pub fn new(device: &WgpuDevice, mesh: &Mesh) -> Self {
        let vertex_buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(mesh.vertices()),
            usage: WgpuBufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(mesh.indices()),
            usage: WgpuBufferUsages::INDEX,
        });

        let index_count = mesh.indices().len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}
