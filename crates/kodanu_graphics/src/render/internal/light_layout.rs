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
pub(crate) struct LightLayout {
    layout: WgpuBindGroupLayout,
}

impl LightLayout {
    pub fn new(device: &WgpuDevice) -> Self {
        let layout = device.create_bind_group_layout(&WgpuBindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
            entries: &[WgpuBindGroupLayoutEntry {
                binding: 0,
                visibility: WgpuShaderStages::FRAGMENT,
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

impl BindGroupLayout for LightLayout {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get(&self) -> &WgpuBindGroupLayout {
        &self.layout
    }

    fn group(&self) -> u32 {
        3
    }
}
