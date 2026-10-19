use crate::{Space, TransformError};

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Quat, Vec3},
    serde::{Deserialize, Serialize},
};

/// Describes the position, rotation, and scale of an entity in 3D space.
///
/// `Transform` uses a translation, rotation, and scale representation.
///
/// # Examples
///
/// ```
/// use {
///     kodanu_math::{Vec3, Quat},
///     kodanu_transform::Transform
/// };
///
/// let transform = Transform::new(
///     Vec3::new(10.0, 0.0, 5.0),
///     Quat::IDENTITY,
///     Vec3::ONE,
/// );
///
/// assert_eq!(transform.position(), Vec3::new(10.0, 0.0, 5.0));
/// assert_eq!(transform.scale(), Vec3::ONE);
/// ```
#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// The position of the entity in world space.
    pub position: Vec3,
    /// The rotation of the entity.
    pub rotation: Quat,
    /// The scale of the entity, applied before rotation.
    pub scale: Vec3,
}

impl Default for Transform {
    /// Creates an identity transform.
    ///
    /// The default transform has zero position, identity rotation,
    /// and unit scale.
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl Transform {
    /// Creates a transform from a position, rotation, and scale.
    ///
    /// The values are stored as given. The rotation is not normalized and
    /// no validation is performed. Use [`Self::try_new`] to normalize the
    /// rotation and reject invalid input.
    #[must_use]
    pub const fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Creates a transform from a position, rotation, and scale, or `None`
    /// if the position or scale is not finite or the rotation cannot be
    /// normalized.
    ///
    /// The rotation is normalized before being stored.
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
    /// Returns the transformation matrix.
    ///
    /// The matrix combines this transform's scale, rotation, and translation.
    #[inline]
    #[must_use]
    pub fn matrix_from_scale_rotation_translation(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Returns the inverse transformation matrix without checking whether
    /// the matrix is invertible.
    ///
    /// This is useful for transforming world-space coordinates into the
    /// local space represented by this transform.
    ///
    /// A singular matrix yields [`Mat4::IDENTITY`] instead of an inverse.
    /// Use [`Self::try_inverse_matrix`] to detect singular matrices.
    #[inline]
    #[must_use]
    pub fn unchecked_inverse_matrix(&self) -> Mat4 {
        self.matrix_from_scale_rotation_translation().inverse()
    }

    /// Returns the inverse transformation matrix, or `None` if the matrix is not invertible.
    ///
    /// This is useful for transforming world-space coordinates into the local space
    /// represented by this transform.
    ///
    /// Returns `None` if the transformation matrix is singular.
    #[inline]
    #[must_use]
    pub fn try_inverse_matrix(&self) -> Option<Mat4> {
        self.matrix_from_scale_rotation_translation().try_inverse()
    }

    /// Transforms a point from local space into world space.
    ///
    /// The point is scaled, rotated, and then translated.
    #[inline]
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    /// Transforms a point from local space into world space, or `None` if
    /// the point or the resulting world-space point is not finite.
    ///
    /// This is the validating version of [`Self::transform_point`].
    #[inline]
    #[must_use]
    pub fn try_transform_point(&self, point: Vec3) -> Option<Vec3> {
        if !point.is_finite() {
            return None;
        }

        let result = self.transform_point(point);

        result.is_finite().then_some(result)
    }

    /// Sets the world-space position.
    ///
    /// Returns `false` and leaves the transform unchanged if the position
    /// is not finite.
    #[inline]
    #[must_use]
    pub const fn try_set_position(&mut self, position: Vec3) -> bool {
        if !position.is_finite() {
            return false;
        }

        self.position = position;

        true
    }

    /// Sets the rotation.
    ///
    /// The rotation is normalized before being stored. Returns `false`
    /// and leaves the transform unchanged if the rotation cannot be
    /// normalized.
    #[inline]
    #[must_use]
    pub fn try_set_rotation(&mut self, rotation: Quat) -> bool {
        let Some(rotation) = rotation.try_normalize() else {
            return false;
        };

        self.rotation = rotation;

        true
    }

    /// Sets the position and rotation.
    ///
    /// The current scale is preserved. The values are stored as given.
    /// Use [`Self::try_set_position_and_rotation`] to validate the input.
    #[inline]
    pub const fn set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    /// Sets the position and rotation.
    ///
    /// The rotation is normalized before being stored. Returns `false`
    /// and leaves the transform unchanged if the position or rotation is
    /// not finite or the rotation cannot be normalized.
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

    /// Sets the scale.
    ///
    /// Returns `false` and leaves the transform unchanged if the scale
    /// is not finite.
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
    /// Returns the forward direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::FORWARD
    }

    /// Returns the forward direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_forward(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let forward = rotation * Vec3::FORWARD;

        forward.is_finite().then_some(forward)
    }

    /// Returns the backward direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::BACKWARD
    }

    /// Returns the backward direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_backward(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let backward = rotation * Vec3::BACKWARD;

        backward.is_finite().then_some(backward)
    }

    /// Returns the right direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::RIGHT
    }

    /// Returns the right direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_right(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let right = rotation * Vec3::RIGHT;

        right.is_finite().then_some(right)
    }

    /// Returns the left direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::LEFT
    }

    /// Returns the left direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_left(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let left = rotation * Vec3::LEFT;

        left.is_finite().then_some(left)
    }

    /// Returns the up direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::UP
    }

    /// Returns the up direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_up(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let up = rotation * Vec3::UP;

        up.is_finite().then_some(up)
    }

    /// Returns the down direction of the transform in world space.
    #[inline]
    #[must_use]
    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::DOWN
    }

    /// Returns the down direction of the transform in world space, or
    /// `None` if the rotation cannot be normalized or the result is not
    /// finite.
    #[inline]
    #[must_use]
    pub fn try_down(&self) -> Option<Vec3> {
        let rotation = self.rotation.try_normalize()?;
        let down = rotation * Vec3::DOWN;

        down.is_finite().then_some(down)
    }
}

impl Transform {
    /// Translates the transform by the given translation.
    ///
    /// In [`crate::Space::Global`] the translation is applied as is, in
    /// [`crate::Space::Local`] it is rotated by the transform's current rotation
    /// first.
    ///
    /// The translation is used as given. Use [`Self::try_translate`] to
    /// reject invalid input.
    ///
    /// # Examples
    ///
    /// ```
    /// use {kodanu_math::*, kodanu_transform::*};
    ///
    /// let mut transform = Transform::from_rotation(Quat::from_axis_angle(Vec3::UP, 90.0));
    ///
    /// transform.translate(Vec3::RIGHT, Space::Local);
    ///
    /// let rotation = Quat::from_axis_angle(Vec3::UP, 90.0);
    ///
    /// assert_eq!(transform.position(), rotation * Vec3::RIGHT);
    /// ```
    #[inline]
    pub fn translate(&mut self, translation: Vec3, space: Space) {
        match space {
            Space::Global => self.position += translation,
            Space::Local => self.position += self.rotation * translation,
        }
    }

    /// Translates the transform, or returns an error and leaves the
    /// transform unchanged if the input or the resulting position is
    /// invalid.
    ///
    /// Returns [`crate::TransformError::NonFiniteTranslation`] if the translation
    /// is not finite, [`crate::TransformError::InvalidRotation`] if `space` is
    /// [`crate::Space::Local`] and the rotation cannot be normalized, and
    /// [`crate::TransformError::NonFinitePosition`] if the resulting position is
    /// not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use {kodanu_math::*, kodanu_transform::*};
    ///
    /// let mut transform = Transform::default();
    ///
    /// assert_eq!(transform.try_translate(Vec3::ONE, Space::Global), Ok(()));
    /// assert_eq!(
    ///     transform.try_translate(Vec3::new(f32::NAN, 0.0, 0.0), Space::Global),
    ///     Err(TransformError::NonFiniteTranslation),
    /// );
    /// ```
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

    /// Rotates the transform around the given axis by the given angle.
    ///
    /// The angle is in degrees and the axis is normalized before use.
    /// In [`crate::Space::Global`] the rotation is applied before the transform's
    /// current rotation, in [`crate::Space::Local`] after it.
    ///
    /// The values are used as given. Use [`Self::try_rotate`] to reject
    /// invalid input.
    ///
    /// # Examples
    ///
    /// ```
    /// use {kodanu_math::*, kodanu_transform::*};
    ///
    /// let mut transform = Transform::default();
    ///
    /// transform.rotate(Vec3::UP, 90.0, Space::Global);
    ///
    /// let delta = Quat::from_axis_angle(Vec3::UP, 90.0);
    ///
    /// assert_eq!(transform.rotation(), delta);
    /// ```
    #[inline]
    pub fn rotate(&mut self, axis: Vec3, angle: f32, space: Space) {
        match space {
            Space::Global => {
                self.rotation = Quat::from_axis_angle(axis.normalize(), angle) * self.rotation
            }
            Space::Local => self.rotation *= Quat::from_axis_angle(axis.normalize(), angle),
        }
    }

    /// Rotates the transform, or returns an error and leaves the
    /// transform unchanged if the input or the resulting rotation is
    /// invalid.
    ///
    /// Returns [`crate::TransformError::NonFiniteAxis`] if the axis is not
    /// finite, [`crate::TransformError::NonFiniteAngle`] if the angle is not
    /// finite, [`crate::TransformError::ZeroAxis`] if the axis is zero, and
    /// [`crate::TransformError::InvalidRotation`] if the transform's rotation or
    /// the resulting rotation cannot be normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use {kodanu_math::*, kodanu_transform::*};
    ///
    /// let mut transform = Transform::default();
    ///
    /// assert_eq!(transform.try_rotate(Vec3::UP, 90.0, Space::Global), Ok(()));
    /// assert_eq!(
    ///     transform.try_rotate(Vec3::ZERO, 90.0, Space::Global),
    ///     Err(TransformError::ZeroAxis),
    /// );
    /// ```
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
    /// Creates a transform with the given position.
    ///
    /// Rotation is [`Quat::IDENTITY`] and scale is [`Vec3::ONE`].
    #[must_use]
    pub const fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    /// Creates a transform with the given position, or `None` if the
    /// position is not finite.
    ///
    /// Rotation is [`Quat::IDENTITY`] and scale is [`Vec3::ONE`].
    /// This is the validating version of [`Self::from_position`].
    #[must_use]
    pub fn try_from_position(position: Vec3) -> Option<Self> {
        Self::try_new(position, Quat::IDENTITY, Vec3::ONE)
    }

    /// Creates a transform with the given rotation.
    ///
    /// Position is [`Vec3::ZERO`] and scale is [`Vec3::ONE`].
    #[must_use]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    /// Creates a transform with the given rotation, or `None` if the
    /// rotation cannot be normalized.
    ///
    /// Position is [`Vec3::ZERO`] and scale is [`Vec3::ONE`].
    /// This is the validating version of [`Self::from_rotation`].
    #[must_use]
    pub fn try_from_rotation(rotation: Quat) -> Option<Self> {
        Self::try_new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    /// Creates a transform with the given scale.
    ///
    /// Position is [`Vec3::ZERO`] and rotation is [`Quat::IDENTITY`].
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    /// Creates a transform with the given scale, or `None` if the scale
    /// is not finite.
    ///
    /// Position is [`Vec3::ZERO`] and rotation is [`Quat::IDENTITY`].
    /// This is the validating version of [`Self::from_scale`].
    #[must_use]
    pub fn try_from_scale(scale: Vec3) -> Option<Self> {
        Self::try_new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    /// Creates a transform with the given position and rotation.
    ///
    /// Scale is initialized to [`Vec3::ONE`].
    #[must_use]
    pub const fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
    }

    /// Creates a transform with the given position and rotation, or `None`
    /// if the position is not finite or the rotation cannot be normalized.
    ///
    /// Scale is initialized to [`Vec3::ONE`].
    /// This is the validating version of [`Self::from_position_rotation`].
    #[must_use]
    pub fn try_from_position_rotation(position: Vec3, rotation: Quat) -> Option<Self> {
        Self::try_new(position, rotation, Vec3::ONE)
    }
}
