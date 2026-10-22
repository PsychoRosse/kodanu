use crate::{AssetServer, GpuAssetCache, RenderQueue};

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct MeshUploadPlugin;

impl Plugin for MeshUploadPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(RenderQueue::with_capacity(1024))
            .with_res(GpuAssetCache::with_capacity(1024))
            .with_res(AssetServer::with_capacity(1024))
            .with_system(Stage::PreRender, RenderQueue::mesh_upload_system);
    }
}
