use crate::Backend;

use wgpu::{
    BackendOptions as WgpuBackendOptions, Backends as WgpuBackends,
    InstanceDescriptor as WgpuInstanceDescriptor, InstanceFlags as WgpuInstanceFlags,
    MemoryBudgetThresholds as WgpuMemoryBudgetThresholds,
};

use {kodanu_math::SurfaceSize, kodanu_window::SurfaceTarget};

pub struct InstanceDescriptor {
    descriptor: WgpuInstanceDescriptor,
    target: SurfaceTarget,
    size: SurfaceSize,
}

impl InstanceDescriptor {
    pub fn with(backend: Backend, target: SurfaceTarget, size: SurfaceSize) -> Self {
        Self {
            descriptor: WgpuInstanceDescriptor {
                backends: WgpuBackends::from(backend),
                flags: WgpuInstanceFlags::default(),
                memory_budget_thresholds: WgpuMemoryBudgetThresholds::default(),
                backend_options: WgpuBackendOptions::default(),
                display: None,
            },
            target,
            size,
        }
    }
}

impl InstanceDescriptor {
    pub const fn surface_target(&self) -> SurfaceTarget {
        self.target
    }

    pub const fn size(&self) -> SurfaceSize {
        self.size
    }
}

impl From<InstanceDescriptor> for WgpuInstanceDescriptor {
    fn from(value: InstanceDescriptor) -> Self {
        let descriptor = value.descriptor;

        Self {
            backends: descriptor.backends,
            flags: descriptor.flags,
            memory_budget_thresholds: descriptor.memory_budget_thresholds,
            backend_options: descriptor.backend_options,
            display: descriptor.display,
        }
    }
}
