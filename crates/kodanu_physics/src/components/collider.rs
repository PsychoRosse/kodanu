use crate::rapier3d;

use {kodanu_ecs::Component, kodanu_macros::Getters, kodanu_math::Vec3};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Getters, Component, Debug)]
pub struct Collider {
    #[getter(copy, must_use)]
    handle: Option<rapier3d::ColliderHandle>,
    #[getter(must_use)]
    shape: rapier3d::ColliderShape,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            handle: None,
            shape: rapier3d::ColliderShape::cuboid(0.5, 0.5, 0.5),
        }
    }
}

impl Collider {
    pub fn cube(size: Vec3) -> Self {
        let half = size * 0.5;

        Self::from(rapier3d::ColliderShape::cuboid(half.x, half.y, half.z))
    }

    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self::from(rapier3d::ColliderShape::triangle(
            rapier3d::Vec3::new(a.x, a.y, a.z),
            rapier3d::Vec3::new(b.x, b.y, b.z),
            rapier3d::Vec3::new(c.x, c.y, c.z),
        ))
    }

    pub fn sphere(radius: f32) -> Self {
        Self::from(rapier3d::ColliderShape::ball(radius * 0.5))
    }
}

impl Collider {
    pub(crate) fn builder(&self) -> rapier3d::ColliderBuilder {
        rapier3d::ColliderBuilder::new(self.shape.clone())
    }

    pub(crate) fn set_handle(&mut self, handle: Option<rapier3d::ColliderHandle>) {
        self.handle = handle
    }
}

impl From<rapier3d::ColliderShape> for Collider {
    fn from(value: rapier3d::ColliderShape) -> Self {
        Self {
            handle: None,
            shape: value,
        }
    }
}
