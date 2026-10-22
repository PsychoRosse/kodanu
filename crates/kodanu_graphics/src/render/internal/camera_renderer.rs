use crate::{CameraLayout, CameraUniform, Instance, PipelineLayoutStorage};

use {
    bytemuck::bytes_of,
    kodanu_camera::{ActiveCamera, Camera},
    kodanu_ecs::{Commands, EventReader, Query, Read, Res, Resource, Write},
    kodanu_macros::Getters,
    kodanu_math::Mat4,
    kodanu_math::SurfaceSize,
    kodanu_transform::Transform,
};

use wgpu::{
    BindGroup as WgpuBindGroup, BindGroupDescriptor as WgpuBindGroupDescriptor,
    BindGroupEntry as WgpuBindGroupEntry, BindGroupLayout as WgpuBindGroupLayout,
    Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice,
    Queue as WgpuQueue,
};

use wgpu::util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt};

#[derive(Getters, Resource)]
pub(crate) struct CameraRenderer {
    #[getter()]
    buffer: WgpuBuffer,
    #[getter()]
    bind_group: WgpuBindGroup,
}

impl CameraRenderer {
    pub fn new(device: &WgpuDevice, layout: &WgpuBindGroupLayout) -> Self {
        let buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytes_of(&CameraUniform::from(Mat4::IDENTITY)),
            usage: WgpuBufferUsages::UNIFORM | WgpuBufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&WgpuBindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout,
            entries: &[WgpuBindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl CameraRenderer {
    pub fn camera_startup_system(
        commands: Commands,
        instance: Res<Instance>,
        storage: Res<PipelineLayoutStorage>,
    ) {
        commands.with_res(CameraRenderer::new(
            instance.device(),
            storage.get_now::<CameraLayout>(),
        ));
    }

    pub fn camera_update_system(
        instance: Res<Instance>,
        render: Res<CameraRenderer>,
        event: EventReader<SurfaceSize>,
        query: Query<(Read<Transform>, Write<Camera>, Read<ActiveCamera>)>,
    ) {
        for (transform, camera, _) in query {
            for event in event.iter() {
                camera.set_aspect_from_surface(*event);
            }

            render.update(
                instance.queue(),
                camera.view_projection_matrix(transform.unchecked_inverse_matrix()),
            );
        }
    }
}

impl CameraRenderer {
    pub fn update(&self, queue: &WgpuQueue, view_proj: Mat4) {
        queue.write_buffer(&self.buffer, 0, bytes_of(&CameraUniform::from(view_proj)));
    }
}
