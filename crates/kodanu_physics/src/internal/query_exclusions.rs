use crate::rapier3d;

use crate::{Collider, RigidBody};

#[derive(Clone, Copy)]
pub struct QueryExclusions<'q> {
    pub colliders: &'q [&'q Collider],
    pub rigid_bodies: &'q [&'q RigidBody],
}

impl QueryExclusions<'_> {
    pub fn allows(&self, handle: rapier3d::ColliderHandle, collider: &rapier3d::Collider) -> bool {
        if self
            .colliders
            .iter()
            .any(|excluded| excluded.handle() == Some(handle))
        {
            return false;
        }

        if let Some(parent) = collider.parent()
            && self
                .rigid_bodies
                .iter()
                .any(|excluded| excluded.handle() == Some(parent))
        {
            return false;
        }

        true
    }
}
