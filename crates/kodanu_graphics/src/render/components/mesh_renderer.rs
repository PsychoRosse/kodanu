use crate::{Color, AssetHandle, Material, Mesh};

use {kodanu_ecs::Component, kodanu_macros::Getters};

#[derive(Getters, Component, Debug, Clone)]
pub struct MeshRenderer {
    #[getter(copy)]
    mesh_handle: Option<AssetHandle<Mesh>>,
    pending_mesh: Option<Mesh>,
    #[getter(copy)]
    material_handle: Option<AssetHandle<Material>>,
    pending_material: Option<Material>,
}

impl MeshRenderer {
    pub fn new(mesh: Option<Mesh>, material: Option<Material>) -> Self {
        Self {
            mesh_handle: None,
            pending_mesh: mesh,
            material_handle: None,
            pending_material: material,
        }
    }
}

impl MeshRenderer {
    pub fn cube(color: Color) -> Self {
        Self::new(Some(Mesh::cube()), Some(Material::from(color)))
    }
}

impl MeshRenderer {
    pub const fn take_mesh_pending(&mut self) -> Option<Mesh> {
        self.pending_mesh.take()
    }

    pub const fn take_material_pending(&mut self) -> Option<Material> {
        self.pending_material.take()
    }
}

impl MeshRenderer {
    pub(crate) const fn set_mesh_handle(&mut self, handle: Option<AssetHandle<Mesh>>) {
        self.mesh_handle = handle
    }

    pub(crate) const fn set_material_handle(&mut self, handle: Option<AssetHandle<Material>>) {
        self.material_handle = handle
    }
}
