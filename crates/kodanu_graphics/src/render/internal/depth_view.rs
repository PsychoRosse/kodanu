use crate::Instance;

use wgpu::{
    Device as WgpuDevice, Extent3d as WgpuExtent3d, Texture as WgpuTexture,
    TextureDescriptor as WgpuTextureDescriptor, TextureDimension as WgpuTextureDimension,
    TextureFormat as WgpuTextureFormat, TextureUsages as WgpuTextureUsages,
    TextureView as WgpuTextureView, TextureViewDescriptor as WgpuTextureViewDescriptor,
};

use {
    kodanu_ecs::{Commands, EventReader, Res, ResMut, Resource},
    kodanu_macros::Getters,
    kodanu_math::SurfaceSize,
};

#[derive(Getters, Resource)]
pub(crate) struct DepthView {
    #[getter()]
    texture: WgpuTexture,
    #[getter()]
    view: WgpuTextureView,
}

impl DepthView {
    pub fn new(device: &WgpuDevice, size: SurfaceSize) -> Self {
        let texture = device.create_texture(&WgpuTextureDescriptor {
            label: Some("Depth View"),
            size: WgpuExtent3d {
                width: size.width(),
                height: size.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: WgpuTextureDimension::D2,
            format: WgpuTextureFormat::Depth32Float,
            usage: WgpuTextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&WgpuTextureViewDescriptor::default());

        Self { texture, view }
    }
}

impl DepthView {
    pub fn depth_view_startup_system(command: Commands, instance: Res<Instance>) {
        command.with_res(DepthView::new(instance.device(), instance.size()));
    }

    pub fn depth_view_system(
        event: EventReader<SurfaceSize>,
        instance: Res<Instance>,
        mut depth: ResMut<DepthView>,
    ) {
        for _ in event.iter() {
            depth.resize(instance.device(), instance.size());
        }
    }
}

impl DepthView {
    pub fn resize(&mut self, device: &WgpuDevice, size: SurfaceSize) {
        let depth = Self::new(device, size);

        self.texture = depth.texture;
        self.view = depth.view;
    }
}
