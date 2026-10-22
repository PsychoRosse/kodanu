use bitflags::bitflags;

use wgpu::Backends as WgpuBackends;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Backend: u32 {
        const VULKAN = 1 << 0;
        const METAL = 1 << 1;
        const DX12 = 1 << 2;
        const WEBGPU = 1 << 3;

        const AUTO = Self::VULKAN.bits()
                    | Self::METAL.bits()
                    | Self::DX12.bits()
                    | Self::WEBGPU.bits();
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::VULKAN | Self::METAL | Self::DX12
    }
}

impl From<Backend> for WgpuBackends {
    fn from(value: Backend) -> Self {
        let mut backends = WgpuBackends::empty();

        if value.contains(Backend::VULKAN) {
            backends |= WgpuBackends::VULKAN;
        }
        if value.contains(Backend::METAL) {
            backends |= WgpuBackends::METAL;
        }
        if value.contains(Backend::DX12) {
            backends |= WgpuBackends::DX12;
        }
        if value.contains(Backend::WEBGPU) {
            backends |= WgpuBackends::BROWSER_WEBGPU;
        }

        backends
    }
}
