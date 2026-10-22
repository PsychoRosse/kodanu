use crate::{Material, MaterialUniform};

use wgpu::{
    BindGroup as WgpuBindGroup, BindGroupDescriptor as WgpuBindGroupDescriptor,
    BindGroupEntry as WgpuBindGroupEntry, BindGroupLayout as WgpuBindGroupLayout,
    Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice,
};

use wgpu::util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt};

use {bytemuck::bytes_of, kodanu_macros::Getters};

#[derive(Getters, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GpuMaterial {
    #[getter()]
    buffer: WgpuBuffer,
    #[getter()]
    bind_group: WgpuBindGroup,
}

impl GpuMaterial {
    pub fn new(device: &WgpuDevice, layout: &WgpuBindGroupLayout, material: &Material) -> Self {
        let buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytes_of(&MaterialUniform::from(material)),
            usage: WgpuBufferUsages::UNIFORM | WgpuBufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&WgpuBindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: &[WgpuBindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}
