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
pub(crate) struct MsaaView {
    texture: WgpuTexture,
    #[getter()]
    view: WgpuTextureView,
}

impl MsaaView {
    #[inline(always)]
    pub fn new(device: &WgpuDevice, size: SurfaceSize, format: WgpuTextureFormat) -> Self {
        let texture = device.create_texture(&WgpuTextureDescriptor {
            label: Some("Msaa View"),
            size: WgpuExtent3d {
                width: size.width(),
                height: size.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: WgpuTextureDimension::D2,
            format,
            usage: WgpuTextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&WgpuTextureViewDescriptor::default());

        Self { texture, view }
    }
}

impl MsaaView {
    pub fn msaa_view_startup(commands: Commands, instance: Res<Instance>) {
        commands.with_res(MsaaView::new(
            instance.device(),
            instance.size(),
            instance.format(),
        ));
    }

    #[inline(always)]
    pub fn msaa_view_system(
        event: EventReader<SurfaceSize>,
        instance: Res<Instance>,
        mut msaa: ResMut<MsaaView>,
    ) {
        for _ in event.iter() {
            msaa.resize(instance.device(), instance.size(), instance.format());
        }
    }
}

impl MsaaView {
    #[inline]
    pub fn resize(&mut self, device: &WgpuDevice, size: SurfaceSize, format: WgpuTextureFormat) {
        let msaa = Self::new(device, size, format);

        self.texture = msaa.texture;
        self.view = msaa.view;
    }
}
