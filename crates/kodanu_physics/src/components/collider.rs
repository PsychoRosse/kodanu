use rapier3d::prelude::{
    ColliderBuilder as RapierColliderBuilder, ColliderHandle as RapierColliderHandle,
    ColliderShape as RapierColliderShape,
};

pub use rapier3d::math::Vec3 as RapierVec3;

use {kodanu_ecs::Component, kodanu_math::Vec3};

#[derive(Component, Debug)]
pub struct Collider {
    handle: Option<RapierColliderHandle>,
    shape: RapierColliderShape,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            handle: None,
            shape: RapierColliderShape::cuboid(0.5, 0.5, 0.5),
        }
    }
}

impl Collider {
    pub fn cube(size: Vec3) -> Self {
        let half = size * 0.5;

        Self::from(RapierColliderShape::cuboid(half.x, half.y, half.z))
    }

    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self::from(RapierColliderShape::triangle(
            RapierVec3::new(a.x, a.y, a.z),
            RapierVec3::new(b.x, b.y, b.z),
            RapierVec3::new(c.x, c.y, c.z),
        ))
    }

    pub fn sphere(radius: f32) -> Self {
        Self::from(RapierColliderShape::ball(radius * 0.5))
    }
}

impl Collider {
    #[inline]
    pub fn handle(&self) -> Option<RapierColliderHandle> {
        self.handle
    }
    #[inline]
    pub fn shape(&self) -> &RapierColliderShape {
        &self.shape
    }
}

impl Collider {
    pub(crate) fn builder(&self) -> RapierColliderBuilder {
        RapierColliderBuilder::new(self.shape.clone())
    }

    pub(crate) fn set_handle(&mut self, handle: Option<RapierColliderHandle>) {
        self.handle = handle
    }
}

impl From<RapierColliderShape> for Collider {
    fn from(value: RapierColliderShape) -> Self {
        Self {
            handle: None,
            shape: value,
        }
    }
}
