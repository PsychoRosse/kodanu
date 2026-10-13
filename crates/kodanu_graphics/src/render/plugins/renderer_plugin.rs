use crate::{
    CameraRenderer, DepthView, Instance, LightRenderer, MsaaView, RenderPass, RenderPipeline,
    RenderQueue,
};

use wgpu::{
    CurrentSurfaceTexture as WgpuCurrentSurfaceTexture, IndexFormat as WgpuIndexFormat,
    SurfaceTexture as WgpuSurfaceTexture, TextureViewDescriptor as WgpuTextureViewDescriptor,
};

use kodanu_ecs::{Plugin, Registry, Res, ResMut, Stage};

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::Render, renderer_system);
    }
}

fn renderer_system(
    mut instance: ResMut<Instance>,
    pipeline: Res<RenderPipeline>,
    camera: Res<CameraRenderer>,
    light: Res<LightRenderer>,
    depth: Res<DepthView>,
    msaa: Res<MsaaView>,
    queue: Res<RenderQueue>,
) {
    match instance.acquire_frame() {
        WgpuCurrentSurfaceTexture::Success(frame) => {
            draw_frame(
                &instance, frame, &pipeline, &camera, &light, &depth, &msaa, &queue,
            );
        }
        WgpuCurrentSurfaceTexture::Suboptimal(frame) => {
            draw_frame(
                &instance, frame, &pipeline, &camera, &light, &depth, &msaa, &queue,
            );

            let size = instance.size();

            instance.resize(size);
        }
        WgpuCurrentSurfaceTexture::Validation => {
            println!("Render validation error");
        }
        WgpuCurrentSurfaceTexture::Lost => {
            println!("Lost device!");
        }
        _ => {
            let size = instance.size();

            instance.resize(size);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_frame(
    instance: &Instance,
    frame: WgpuSurfaceTexture,
    pipeline: &RenderPipeline,
    camera: &CameraRenderer,
    light: &LightRenderer,
    depth: &DepthView,
    msaa: &MsaaView,
    queue: &RenderQueue,
) {
    let view = frame
        .texture
        .create_view(&WgpuTextureViewDescriptor::default());

    let mut encoder = instance.create_encoder();

    {
        let mut render_pass = RenderPass::begin(&mut encoder, &view, depth.view(), msaa.view());

        render_pass.set_pipeline(pipeline.get());

        render_pass.set_bind_group(0, camera.bind_group(), &[]);
        render_pass.set_bind_group(3, light.bind_group(), &[]);

        for item in queue.iter() {
            render_pass.set_bind_group(1, item.material().bind_group(), &[]);
            render_pass.set_bind_group(2, item.model().bind_group(), &[]);

            render_pass.set_vertex_buffer(0, item.mesh().vertex_buffer().slice(..));

            render_pass.set_index_buffer(
                item.mesh().index_buffer().slice(..),
                WgpuIndexFormat::Uint32,
            );

            render_pass.draw_indexed(0..item.mesh().index_count(), 0, 0..1);
        }
    }

    instance.present(encoder, frame);
}
