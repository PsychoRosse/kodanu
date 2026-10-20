use crate::{Space, TransformError};

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Quat, Vec3},
    serde::{Deserialize, Serialize},
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl Transform {
    #[must_use]
    pub const fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    #[must_use]
    pub fn try_new(position: Vec3, rotation: Quat, scale: Vec3) -> Option<Self> {
        if !position.is_finite() || !scale.is_finite() {
            return None;
        }

        let rotation = rotation.try_normalize()?;

        Some(Self {
            position,
            rotation,
            scale,
        })
    }
}

impl Transform {
    #[inline]
    #[must_use]
    pub fn matrix_from_scale_rotation_translation(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_inverse_matrix(&self) -> Mat4 {
        self.matrix_from_scale_rotation_translation().inverse()
    }

    #[inline]
    #[must_use]
    pub fn try_inverse_matrix(&self) -> Option<Mat4> {
        self.matrix_from_scale_rotation_translation().try_inverse()
    }

    #[inline]
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    #[inline]
    #[must_use]
    pub fn try_transform_point(&self, point: Vec3) -> Option<Vec3> {
        if !point.is_finite() {
            return None;
        }

        let result = self.transform_point(point);

        result.is_finite().then_some(result)
    }

    #[inline]
    #[must_use]
    pub const fn try_set_position(&mut self, position: Vec3) -> bool {
        if !position.is_finite() {
            return false;
        }

        self.position = position;

        true
    }

    #[inline]
    #[must_use]
    pub fn try_set_rotation(&mut self, rotation: Quat) -> bool {
        let Some(rotation) = rotation.try_normalize() else {
            return false;
        };

        self.rotation = rotation;

        true
    }

    #[inline]
    pub const fn set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    #[inline]
    #[must_use]
    pub fn try_set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) -> bool {
        if !position.is_finite() || !rotation.is_finite() {
            return false;
        }

        let Some(rotation) = rotation.try_normalize() else {
            return false;
        };

        self.set_position_and_rotation(position, rotation);

        true
    }

    #[inline]
    #[must_use]
    pub const fn try_set_scale(&mut self, scale: Vec3) -> bool {
        if !scale.is_finite() {
            return false;
        }

        self.scale = scale;

        true
    }
}

impl Transform {
    #[inline]
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::FORWARD
    }

    #[inline]
    #[must_use]
    pub fn try_forward(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let forward = rotation * Vec3::FORWARD;

        forward.is_finite().then_some(forward)
    }

    #[inline]
    #[must_use]
    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::BACKWARD
    }

    #[inline]
    #[must_use]
    pub fn try_backward(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let backward = rotation * Vec3::BACKWARD;

        backward.is_finite().then_some(backward)
    }

    #[inline]
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::RIGHT
    }

    #[inline]
    #[must_use]
    pub fn try_right(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let right = rotation * Vec3::RIGHT;

        right.is_finite().then_some(right)
    }

    #[inline]
    #[must_use]
    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::LEFT
    }

    #[inline]
    #[must_use]
    pub fn try_left(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let left = rotation * Vec3::LEFT;

        left.is_finite().then_some(left)
    }

    #[inline]
    #[must_use]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::UP
    }

    #[inline]
    #[must_use]
    pub fn try_up(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let up = rotation * Vec3::UP;

        up.is_finite().then_some(up)
    }

    #[inline]
    #[must_use]
    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::DOWN
    }

    #[inline]
    #[must_use]
    pub fn try_down(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let down = rotation * Vec3::DOWN;

        down.is_finite().then_some(down)
    }
}

impl Transform {
    #[inline]
    pub fn translate(&mut self, translation: Vec3, space: Space) {
        match space {
            Space::Global => self.position += translation,
            Space::Local => self.position += self.rotation * translation,
        }
    }

    #[inline]
    pub fn try_translate(&mut self, translation: Vec3, space: Space) -> Result<(), TransformError> {
        if !translation.is_finite() {
            return Err(TransformError::NonFiniteTranslation);
        }

        let position = match space {
            Space::Global => self.position + translation,
            Space::Local => {
                let Some(rotation) = self.rotation.try_normalize() else {
                    return Err(TransformError::InvalidRotation);
                };

                self.position + rotation * translation
            }
        };

        if !position.is_finite() {
            return Err(TransformError::NonFinitePosition);
        }

        self.position = position;

        Ok(())
    }

    #[inline]
    pub fn rotate(&mut self, axis: Vec3, angle: f32, space: Space) {
        match space {
            Space::Global => {
                self.rotation = Quat::from_axis_angle(axis.normalize(), angle) * self.rotation
            }
            Space::Local => self.rotation *= Quat::from_axis_angle(axis.normalize(), angle),
        }
    }

    #[inline]
    pub fn try_rotate(
        &mut self,
        axis: Vec3,
        angle: f32,
        space: Space,
    ) -> Result<(), TransformError> {
        if !axis.is_finite() {
            return Err(TransformError::NonFiniteAxis);
        }

        if !angle.is_finite() {
            return Err(TransformError::NonFiniteAngle);
        }

        let Some(axis) = axis.try_normalize() else {
            return Err(TransformError::ZeroAxis);
        };

        if !self.rotation.is_finite() {
            return Err(TransformError::InvalidRotation);
        }

        let delta = Quat::from_axis_angle(axis, angle);

        let rotation = match space {
            Space::Global => delta * self.rotation,
            Space::Local => self.rotation * delta,
        };

        let Some(rotation) = rotation.try_normalize() else {
            return Err(TransformError::InvalidRotation);
        };

        self.rotation = rotation;

        Ok(())
    }
}

impl Transform {
    #[must_use]
    pub const fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    #[must_use]
    pub fn try_from_position(position: Vec3) -> Option<Self> {
        Self::try_new(position, Quat::IDENTITY, Vec3::ONE)
    }

    #[must_use]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    #[must_use]
    pub fn try_from_rotation(rotation: Quat) -> Option<Self> {
        Self::try_new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    #[must_use]
    pub fn try_from_scale(scale: Vec3) -> Option<Self> {
        Self::try_new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    #[must_use]
    pub const fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
    }

    #[must_use]
    pub fn try_from_position_rotation(position: Vec3, rotation: Quat) -> Option<Self> {
        Self::try_new(position, rotation, Vec3::ONE)
    }
}
