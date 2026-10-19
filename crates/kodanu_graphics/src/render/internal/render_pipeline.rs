use crate::{ShaderStorage, Vertex};

use wgpu::{
    BindGroupLayout as WgpuBindGroupLayout, BlendState as WgpuBlendState,
    ColorTargetState as WgpuColorTargetState, ColorWrites as WgpuColorWrites,
    CompareFunction as WgpuCompareFunction, DepthBiasState as WgpuDepthBiasState,
    DepthStencilState as WgpuDepthStencilState, Device as WgpuDevice, Face as WgpuFace,
    FragmentState as WgpuFragmentState, FrontFace as WgpuFrontFace,
    MultisampleState as WgpuMultiSampleState,
    PipelineCompilationOptions as WgpuPipelineCompilationOptions,
    PipelineLayoutDescriptor as WgpuPipelineLayoutDescriptor, PolygonMode as WgpuPolygonMode,
    PrimitiveState as WgpuPrimitiveState, PrimitiveTopology as WgpuPrimitiveTopology,
    RenderPipeline as WgpuRenderPipeline, RenderPipelineDescriptor as WgpuRenderPipelineDescriptor,
    StencilState as WgpuStencilState, TextureFormat as WgpuTextureFormat,
    VertexState as WgpuVertexState,
};

use kodanu_ecs::Resource;

#[derive(Resource)]
pub(crate) struct RenderPipeline {
    pipeline: WgpuRenderPipeline,
}

impl RenderPipeline {
    pub fn new(
        device: &WgpuDevice,
        format: WgpuTextureFormat,
        layouts: &[Option<&WgpuBindGroupLayout>],
    ) -> Self {
        let layout = device.create_pipeline_layout(&WgpuPipelineLayoutDescriptor {
            label: Some("Pipeline Layout Layout"),
            bind_group_layouts: layouts,
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&WgpuRenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: WgpuVertexState {
                module: &ShaderStorage::vertex(device),
                entry_point: Some("main"),
                compilation_options: WgpuPipelineCompilationOptions::default(),
                buffers: &[Some(Vertex::layout())],
            },
            fragment: Some(WgpuFragmentState {
                module: &ShaderStorage::normal(device),
                entry_point: Some("main"),
                compilation_options: WgpuPipelineCompilationOptions::default(),
                targets: &[Some(WgpuColorTargetState {
                    format,
                    blend: Some(WgpuBlendState::REPLACE),
                    write_mask: WgpuColorWrites::ALL,
                })],
            }),
            depth_stencil: Some(WgpuDepthStencilState {
                format: WgpuTextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(WgpuCompareFunction::Less),
                stencil: WgpuStencilState::default(),
                bias: WgpuDepthBiasState::default(),
            }),
            primitive: WgpuPrimitiveState {
                topology: WgpuPrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: WgpuFrontFace::Ccw,
                cull_mode: Some(WgpuFace::Back),
                unclipped_depth: false,
                polygon_mode: WgpuPolygonMode::Fill,
                conservative: false,
            },
            multisample: WgpuMultiSampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self { pipeline }
    }
}

impl RenderPipeline {
    pub fn get(&self) -> &WgpuRenderPipeline {
        &self.pipeline
    }
}
