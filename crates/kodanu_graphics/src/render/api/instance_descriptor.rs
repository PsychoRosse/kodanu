use crate::Backend;

use wgpu::{
    BackendOptions as WgpuBackendOptions, Backends as WgpuBackends,
    InstanceDescriptor as WgpuInstanceDescriptor, InstanceFlags as WgpuInstanceFlags,
    MemoryBudgetThresholds as WgpuMemoryBudgetThresholds,
};

use {kodanu_macros::Getters, kodanu_math::SurfaceSize, kodanu_window::SurfaceTarget};

#[derive(Getters, Debug)]
pub struct InstanceDescriptor {
    descriptor: WgpuInstanceDescriptor,
    #[getter(copy)]
    surface_target: SurfaceTarget,
    #[getter(copy)]
    surface_size: SurfaceSize,
}

impl InstanceDescriptor {
    pub fn with(
        backend: Backend,
        surface_target: SurfaceTarget,
        surface_size: SurfaceSize,
    ) -> Self {
        Self {
            descriptor: WgpuInstanceDescriptor {
                backends: WgpuBackends::from(backend),
                flags: WgpuInstanceFlags::default(),
                memory_budget_thresholds: WgpuMemoryBudgetThresholds::default(),
                backend_options: WgpuBackendOptions::default(),
                display: None,
            },
            surface_target,
            surface_size,
        }
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
