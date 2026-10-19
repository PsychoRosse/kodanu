use crate::Transform;

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Vec3},
    serde::{Deserialize, Serialize},
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GlobalTransform {
    matrix: Mat4,
}

impl GlobalTransform {
    pub const IDENTITY: Self = Self {
        matrix: Mat4::IDENTITY,
    };
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl GlobalTransform {
    #[must_use]
    pub const fn from_matrix(matrix: Mat4) -> Self {
        Self { matrix }
    }

    #[must_use]
    pub fn from_transform(transform: &Transform) -> Self {
        Self::from_matrix(transform.matrix_from_scale_rotation_translation())
    }

    #[must_use]
    pub fn from_parent(parent: &Self, local: &Transform) -> Self {
        Self::from_matrix(parent.matrix * local.matrix_from_scale_rotation_translation())
    }

    #[must_use]
    pub const fn matrix(&self) -> Mat4 {
        self.matrix
    }

    #[must_use]
    pub fn try_inverse_matrix(&self) -> Option<Mat4> {
        self.matrix.try_inverse()
    }

    #[must_use]
    pub fn unchecked_inverse_matrix(&self) -> Mat4 {
        self.matrix.inverse()
    }

    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.matrix.transform_point3(point)
    }

    #[must_use]
    pub fn try_transform_point(&self, point: Vec3) -> Option<Vec3> {
        if !point.is_finite() {
            return None;
        }

        let result = self.transform_point(point);
        result.is_finite().then_some(result)
    }

    #[must_use]
    pub fn try_world_to_local(&self, point: Vec3) -> Option<Vec3> {
        if !point.is_finite() {
            return None;
        }

        let inverse = self.matrix.try_inverse()?;
        let result = inverse.transform_point3(point);

        result.is_finite().then_some(result)
    }

    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.transform_point(Vec3::ZERO)
    }
}
