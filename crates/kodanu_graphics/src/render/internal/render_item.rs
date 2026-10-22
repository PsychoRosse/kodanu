use crate::{GpuMaterial, GpuMesh, GpuModel};

use {kodanu_macros::Getters, std::sync::Arc};

#[derive(Getters, Debug, Clone)]
pub(crate) struct RenderItem {
    #[getter()]
    mesh: Arc<GpuMesh>,
    #[getter()]
    material: Arc<GpuMaterial>,
    #[getter()]
    model: GpuModel,
}

impl RenderItem {
    pub fn new(mesh: Arc<GpuMesh>, material: Arc<GpuMaterial>, model: GpuModel) -> Self {
        Self {
            mesh,
            material,
            model,
        }
    }
}
