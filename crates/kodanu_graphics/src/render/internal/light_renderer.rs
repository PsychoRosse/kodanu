use crate::{DirectLight, Instance, LightLayout, LightUniform, PipelineLayoutStorage};

use wgpu::{
    BindGroup as WgpuBindGroup, BindGroupDescriptor as WgpuBindGroupDescriptor,
    BindGroupEntry as WgpuBindGroupEntry, BindGroupLayout as WgpuBindGroupLayout,
    Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice,
    Queue as WgpuQueue,
};

use wgpu::util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt};

use {
    bytemuck::bytes_of,
    kodanu_ecs::{Commands, Query, Read, Res, Resource},
    kodanu_macros::Getters,
};

#[derive(Getters, Resource)]
pub(crate) struct LightRenderer {
    #[getter()]
    buffer: WgpuBuffer,
    #[getter()]
    bind_group: WgpuBindGroup,
}

impl LightRenderer {
    pub fn new(device: &WgpuDevice, layout: &WgpuBindGroupLayout) -> Self {
        let buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytes_of(&LightUniform::default()),
            usage: WgpuBufferUsages::UNIFORM | WgpuBufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&WgpuBindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout,
            entries: &[WgpuBindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl LightRenderer {
    pub fn light_startup_system(
        commands: Commands,
        instance: Res<Instance>,
        storage: Res<PipelineLayoutStorage>,
    ) {
        commands.with_res(LightRenderer::new(
            instance.device(),
            storage.get_now::<LightLayout>(),
        ));
    }

    #[inline(always)]
    pub fn light_system(
        query: Query<Read<DirectLight>>,
        render: Res<LightRenderer>,
        instance: Res<Instance>,
    ) {
        for light in query {
            render.update(instance.queue(), light);
        }
    }
}

impl LightRenderer {
    #[inline]
    pub fn update(&self, queue: &WgpuQueue, light: &DirectLight) {
        queue.write_buffer(&self.buffer, 0, bytes_of(&LightUniform::from(light)));
    }
}

impl From<&DirectLight> for LightUniform {
    #[inline]
    fn from(value: &DirectLight) -> Self {
        Self::new(
            value.direction().normalize(),
            value.intensity(),
            value.color(),
        )
    }
}
