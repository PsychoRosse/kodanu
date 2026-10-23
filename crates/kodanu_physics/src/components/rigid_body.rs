use crate::rapier3d;

use {kodanu_ecs::Component, kodanu_macros::Getters, kodanu_transform::Transform};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Getters, Component, Debug)]
pub struct RigidBody {
    #[getter(copy, must_use)]
    handle: Option<rapier3d::RigidBodyHandle>,
    #[getter(copy, must_use)]
    body_type: rapier3d::RigidBodyType,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::dynamic()
    }
}

impl RigidBody {
    pub fn dynamic() -> Self {
        Self::from(rapier3d::RigidBodyType::Dynamic)
    }

    pub fn fixed() -> Self {
        Self::from(rapier3d::RigidBodyType::Fixed)
    }

    pub fn kinematic() -> Self {
        Self::from(rapier3d::RigidBodyType::KinematicPositionBased)
    }
}

impl RigidBody {
    pub(crate) fn builder(&self, transform: &Transform) -> rapier3d::RigidBodyBuilder {
        let (axis, angle) = transform.rotation.to_axis_angle();

        let angle = angle.to_radians();

        let position = rapier3d::Vector::new(
            transform.position.x,
            transform.position.y,
            transform.position.z,
        );

        let ang = rapier3d::AngVector::new(axis.x * angle, axis.y * angle, axis.z * angle);

        rapier3d::RigidBodyBuilder::new(self.body_type)
            .translation(position)
            .rotation(ang)
    }

    pub(crate) fn set_handle(&mut self, handle: Option<rapier3d::RigidBodyHandle>) {
        self.handle = handle
    }
}

impl From<rapier3d::RigidBodyType> for RigidBody {
    fn from(value: rapier3d::RigidBodyType) -> Self {
        Self {
            handle: None,
            body_type: value,
        }
    }
}
