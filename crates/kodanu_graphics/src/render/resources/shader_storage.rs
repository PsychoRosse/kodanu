use std::{borrow::Cow, fs::read_to_string};

use wgpu::{
    Device as WgpuDevice, ShaderModule as WgpuShaderModule,
    ShaderModuleDescriptor as WgpuShaderModuleDescriptor, ShaderSource as WgpuShaderSource,
};

pub struct ShaderStorage;

impl ShaderStorage {
    pub fn vertex(device: &WgpuDevice) -> WgpuShaderModule {
        let source: Cow<str> = read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/wgsl/mesh.vert.wgsl"
        ))
        .unwrap()
        .into();

        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: WgpuShaderSource::Wgsl(source),
        })
    }

    pub fn fragment(device: &WgpuDevice) -> WgpuShaderModule {
        let source: Cow<str> = read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/wgsl/mesh.frag.wgsl"
        ))
        .unwrap()
        .into();

        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: WgpuShaderSource::Wgsl(source),
        })
    }

    pub fn normal(device: &WgpuDevice) -> WgpuShaderModule {
        let source: Cow<str> = read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/wgsl/mesh.normal.wgsl"
        ))
        .unwrap()
        .into();

        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: WgpuShaderSource::Wgsl(source),
        })
    }
}
