use crate::ModelUniform;

use wgpu::util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt};

use wgpu::{
    BindGroup as WgpuBindGroup, BindGroupDescriptor as WgpuBindGroupDescriptor,
    BindGroupEntry as WgpuBindGroupEntry, BindGroupLayout as WgpuBindGroupLayout,
    Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice,
    Queue as WgpuQueue,
};

use {bytemuck::bytes_of, kodanu_macros::Getters, kodanu_math::Mat4};

#[derive(Getters, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GpuModel {
    #[getter()]
    buffer: WgpuBuffer,
    #[getter()]
    bind_group: WgpuBindGroup,
}

impl GpuModel {
    pub fn new(device: &WgpuDevice, layout: &WgpuBindGroupLayout, model: Mat4) -> Self {
        let buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytes_of(&ModelUniform::from(model)),
            usage: WgpuBufferUsages::UNIFORM | WgpuBufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&WgpuBindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout,
            entries: &[WgpuBindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl GpuModel {
    pub fn update(&self, queue: &WgpuQueue, matrix: Mat4) {
        queue.write_buffer(&self.buffer, 0, bytes_of(&ModelUniform::from(matrix)));
    }
}
