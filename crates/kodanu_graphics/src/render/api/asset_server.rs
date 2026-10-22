use crate::{AssetStorage, AssetHandle, MeshRenderer};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use kodanu_ecs::{Query, ResMut, Resource, Write};

#[derive(Resource, Default)]
pub struct AssetServer {
    assets: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl AssetServer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            assets: HashMap::with_capacity(capacity),
        }
    }
}

impl AssetServer {
    pub fn add<A: Any + Send + Sync>(&mut self, asset: A) -> AssetHandle<A> {
        self.assets
            .entry(TypeId::of::<A>())
            .or_insert_with(|| Box::new(AssetStorage::<A>::default()))
            .downcast_mut::<AssetStorage<A>>()
            .unwrap()
            .add(asset)
    }

    pub fn get<A: Any + Send + Sync>(&self, handle: AssetHandle<A>) -> Option<&A> {
        self.assets
            .get(&TypeId::of::<A>())
            .and_then(|asset| asset.downcast_ref::<AssetStorage<A>>())?
            .get(handle)
    }
}

impl AssetServer {
    pub(crate) fn asset_system(mut server: ResMut<AssetServer>, query: Query<Write<MeshRenderer>>) {
        for renderer in query {
            if renderer.mesh_handle().is_none()
                && let Some(mesh) = renderer.take_mesh_pending()
            {
                renderer.set_mesh_handle(Some(server.add(mesh)));
            }

            if renderer.material_handle().is_none()
                && let Some(material) = renderer.take_material_pending()
            {
                renderer.set_material_handle(Some(server.add(material)));
            }
        }
    }
}
