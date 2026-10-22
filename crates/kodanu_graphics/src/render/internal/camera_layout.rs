use crate::BindGroupLayout;

use wgpu::{
    BindGroupLayout as WgpuBindGroupLayout,
    BindGroupLayoutDescriptor as WgpuBindGroupLayoutDescriptor,
    BindGroupLayoutEntry as WgpuBindGroupLayoutEntry, BindingType as WgpuBindingType,
    BufferBindingType as WgpuBufferBindingType, Device as WgpuDevice,
    ShaderStages as WgpuShaderStages,
};

use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CameraLayout {
    layout: WgpuBindGroupLayout,
}

impl CameraLayout {
    pub fn new(device: &WgpuDevice) -> Self {
        let layout = device.create_bind_group_layout(&WgpuBindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[WgpuBindGroupLayoutEntry {
                binding: 0,
                visibility: WgpuShaderStages::VERTEX,
                ty: WgpuBindingType::Buffer {
                    ty: WgpuBufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self { layout }
    }
}

impl BindGroupLayout for CameraLayout {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get(&self) -> &WgpuBindGroupLayout {
        &self.layout
    }

    fn group(&self) -> u32 {
        0
    }
}
