use crate::{Collider, RigidBody};

use rapier3d::prelude::{Collider as RapierCollider, ColliderHandle as RapierColliderHandle};

pub struct QueryExclusions<'a> {
    pub colliders: &'a [&'a Collider],
    pub rigid_bodies: &'a [&'a RigidBody],
}

impl QueryExclusions<'_> {
    pub fn allows(&self, handle: RapierColliderHandle, collider: &RapierCollider) -> bool {
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
