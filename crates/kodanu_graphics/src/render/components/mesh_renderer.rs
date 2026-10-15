use crate::{Color, Handle, Material, Mesh};

use kodanu_ecs::Component;

#[derive(Component, Debug)]
pub struct MeshRenderer {
    pending_mesh: Option<Mesh>,
    pending_material: Option<Material>,
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<Material>>,
}

impl MeshRenderer {
    pub fn new(mesh: Option<Mesh>, material: Option<Material>) -> Self {
        Self {
            mesh: None,
            pending_mesh: mesh,
            material: None,
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
    pub const fn mesh_handle(&self) -> Option<Handle<Mesh>> {
        self.mesh
    }

    pub const fn take_mesh_pending(&mut self) -> Option<Mesh> {
        self.pending_mesh.take()
    }

    pub const fn material_handle(&self) -> Option<Handle<Material>> {
        self.material
    }

    pub const fn take_material_pending(&mut self) -> Option<Material> {
        self.pending_material.take()
    }
}
impl MeshRenderer {
    pub(crate) const fn set_mesh_handle(&mut self, handle: Option<Handle<Mesh>>) {
        self.mesh = handle
    }

    pub(crate) const fn set_material_handle(&mut self, handle: Option<Handle<Material>>) {
        self.material = handle
    }
}
