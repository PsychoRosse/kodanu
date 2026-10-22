use crate::{AssetHandle, AssetServer, GpuMaterial, GpuMesh, Material, Mesh};

use {
    kodanu_ecs::Resource,
    std::{collections::HashMap, sync::Arc},
};

#[derive(Resource, Default)]
pub(crate) struct GpuAssetCache {
    meshes: HashMap<AssetHandle<Mesh>, Arc<GpuMesh>>,
    materials: HashMap<AssetHandle<Material>, Arc<GpuMaterial>>,
}

impl GpuAssetCache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            meshes: HashMap::with_capacity(capacity),
            materials: HashMap::with_capacity(capacity),
        }
    }
}

impl GpuAssetCache {
    pub fn retain(&mut self, server: &AssetServer) {
        self.meshes
            .retain(|handle, _| server.get(*handle).is_some());
        self.materials
            .retain(|handle, _| server.get(*handle).is_some());
    }

    pub fn mesh(&self, handle: AssetHandle<Mesh>) -> Option<Arc<GpuMesh>> {
        self.meshes.get(&handle).cloned()
    }

    pub fn insert_mesh(&mut self, handle: AssetHandle<Mesh>, mesh: GpuMesh) -> Arc<GpuMesh> {
        self.meshes
            .entry(handle)
            .or_insert_with(|| Arc::new(mesh))
            .clone()
    }

    pub fn material(&self, handle: AssetHandle<Material>) -> Option<Arc<GpuMaterial>> {
        self.materials.get(&handle).cloned()
    }

    pub fn insert_material(
        &mut self,
        handle: AssetHandle<Material>,
        material: GpuMaterial,
    ) -> Arc<GpuMaterial> {
        self.materials
            .entry(handle)
            .or_insert_with(|| Arc::new(material))
            .clone()
    }
}
