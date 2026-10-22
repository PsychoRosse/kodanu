use wgpu::{
    Color as WgpuColor, CommandEncoder as WgpuCommandEncoder, LoadOp as WgpuLoadOp,
    Operations as WgpuOperations, RenderPass as WgpuRenderPass,
    RenderPassColorAttachment as WgpuRenderPassColorAttachment,
    RenderPassDepthStencilAttachment as WgpuRenderPassDepthStencilAttachment,
    RenderPassDescriptor as WgpuRenderPassDescriptor, StoreOp as WgpuStoreOp,
    TextureView as WgpuTextureView,
};

pub(crate) struct RenderPass;

impl<'r> RenderPass {
    #[inline]
    pub fn begin(
        encoder: &'r mut WgpuCommandEncoder,
        color: &WgpuTextureView,
        depth: &WgpuTextureView,
        msaa: &WgpuTextureView,
    ) -> WgpuRenderPass<'r> {
        encoder.begin_render_pass(&WgpuRenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(WgpuRenderPassColorAttachment {
                view: msaa,
                resolve_target: Some(color),
                depth_slice: None,
                ops: WgpuOperations {
                    load: WgpuLoadOp::Clear(WgpuColor {
                        r: (0.01),
                        g: (0.01),
                        b: (0.01),
                        a: (1.0),
                    }),
                    store: WgpuStoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(WgpuRenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(WgpuOperations {
                    load: WgpuLoadOp::Clear(1.0),
                    store: WgpuStoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        })
    }
}
