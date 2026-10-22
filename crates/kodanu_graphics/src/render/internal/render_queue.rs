use crate::{
    AssetServer, GpuAssetCache, GpuMaterial, GpuMesh, GpuModel, Instance, MaterialLayout,
    MeshRenderer, ModelLayout, PipelineLayoutStorage, RenderItem,
};

use {
    kodanu_ecs::{Query, Read, Res, ResMut, Resource},
    kodanu_transform::Transform,
    std::slice::Iter,
};

#[derive(Resource, Default)]
pub(crate) struct RenderQueue {
    items: Vec<RenderItem>,
}

impl RenderQueue {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }
}

impl RenderQueue {
    #[inline(always)]
    pub fn mesh_upload_system(
        mut queue: ResMut<RenderQueue>,
        mut cache: ResMut<GpuAssetCache>,
        instance: Res<Instance>,
        storage: Res<PipelineLayoutStorage>,
        server: Res<AssetServer>,
        query: Query<(Read<Transform>, Read<MeshRenderer>)>,
    ) {
        queue.clear();

        cache.retain(&server);

        for (transform, object) in query {
            let Some(mesh_handle) = object.mesh_handle() else {
                continue;
            };

            let Some(material_handle) = object.material_handle() else {
                continue;
            };

            let Some(mesh) = server.get(mesh_handle) else {
                continue;
            };

            let Some(material) = server.get(material_handle) else {
                continue;
            };

            let device = instance.device();

            let (material_layout, model_layout) = (
                storage.get_now::<MaterialLayout>(),
                storage.get_now::<ModelLayout>(),
            );

            let gpu_mesh = match cache.mesh(mesh_handle) {
                Some(mesh) => mesh,
                None => cache.insert_mesh(mesh_handle, GpuMesh::new(device, mesh)),
            };

            let gpu_material = match cache.material(material_handle) {
                Some(material) => material,
                None => cache.insert_material(
                    material_handle,
                    GpuMaterial::new(device, material_layout, material),
                ),
            };

            let gpu_model = GpuModel::new(
                device,
                model_layout,
                transform.matrix_from_scale_rotation_translation(),
            );

            queue.push(RenderItem::new(gpu_mesh, gpu_material, gpu_model))
        }
    }
}

impl RenderQueue {
    #[inline]
    pub fn push(&mut self, item: RenderItem) {
        self.items.push(item);
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, RenderItem> {
        self.items.iter()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
