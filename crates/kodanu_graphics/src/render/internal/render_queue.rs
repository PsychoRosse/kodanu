use crate::{
    AssetServer, GpuMaterial, GpuMesh, GpuModel, Instance, MaterialLayout, MeshRenderer,
    ModelLayout, PipelineLayoutStorage, RenderItem,
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
    pub fn mesh_upload_system(
        mut queue: ResMut<RenderQueue>,
        instance: Res<Instance>,
        storage: Res<PipelineLayoutStorage>,
        server: Res<AssetServer>,
        query: Query<(Read<Transform>, Read<MeshRenderer>)>,
    ) {
        queue.clear();

        for (transform, object) in query {
            let Some(mesh_handle) = object.mesh_handle() else {
                continue;
            };

            let Some(mesh) = server.get(mesh_handle) else {
                continue;
            };

            let Some(material_handle) = object.material_handle() else {
                continue;
            };

            let Some(material) = server.get(material_handle) else {
                continue;
            };

            queue.push(RenderItem::new(
                GpuMesh::new(instance.device(), mesh),
                GpuMaterial::new(
                    instance.device(),
                    storage.expect_get::<MaterialLayout>(),
                    material,
                ),
                GpuModel::new(
                    instance.device(),
                    storage.expect_get::<ModelLayout>(),
                    transform.matrix_from_scale_rotation_translation(),
                ),
            ))
        }
    }
}

impl RenderQueue {
    pub fn push(&mut self, item: RenderItem) {
        self.items.push(item);
    }

    pub fn iter(&self) -> Iter<'_, RenderItem> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
