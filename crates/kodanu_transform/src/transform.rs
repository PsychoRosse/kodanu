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
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
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
    /// The value is stored as given. Use [`Self::try_set_position`] to
    /// reject non-finite positions.
    #[inline]
    pub const fn set_position(&mut self, position: Vec3) {
        self.position = position;
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
    /// The value is stored as given. Use [`Self::try_set_rotation`] to
    /// normalize the rotation and reject invalid input.
    #[inline]
    pub const fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
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
    /// The value is stored as given. Use [`Self::try_set_scale`] to
    /// reject non-finite scales.
    #[inline]
    pub const fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
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

    /// Returns the world-space position.
    #[inline]
    #[must_use]
    pub const fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the rotation.
    #[inline]
    #[must_use]
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Returns the scale.
    #[inline]
    #[must_use]
    pub const fn scale(&self) -> Vec3 {
        self.scale
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
    /// In [`Space::Global`] the translation is applied as is, in
    /// [`Space::Local`] it is rotated by the transform's current rotation
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
    /// Returns [`TransformError::NonFiniteTranslation`] if the translation
    /// is not finite, [`TransformError::InvalidRotation`] if `space` is
    /// [`Space::Local`] and the rotation cannot be normalized, and
    /// [`TransformError::NonFinitePosition`] if the resulting position is
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
    /// In [`Space::Global`] the rotation is applied before the transform's
    /// current rotation, in [`Space::Local`] after it.
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
    /// Returns [`TransformError::NonFiniteAxis`] if the axis is not
    /// finite, [`TransformError::NonFiniteAngle`] if the angle is not
    /// finite, [`TransformError::ZeroAxis`] if the axis is zero, and
    /// [`TransformError::InvalidRotation`] if the transform's rotation or
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_identity() {
        let transform = Transform::default();

        assert_eq!(transform.position(), Vec3::ZERO);
        assert_eq!(transform.rotation(), Quat::IDENTITY);
        assert_eq!(transform.scale(), Vec3::ONE);
    }

    #[test]
    fn new_stores_components() {
        let position = Vec3::new(1.0, -2.0, 3.5);
        let rotation = Quat::from_axis_angle(Vec3::UP, 45.0);
        let scale = Vec3::new(2.0, 0.5, 3.0);
        let transform = Transform::new(position, rotation, scale);

        assert_eq!(transform.position(), position);
        assert_eq!(transform.rotation(), rotation);
        assert_eq!(transform.scale(), scale);
    }

    #[test]
    fn constructors_initialize_expected_components() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::from_axis_angle(Vec3::FORWARD, 30.0);
        let scale = Vec3::new(2.0, 2.0, 2.0);

        let from_position = Transform::from_position(position);

        assert_eq!(from_position.position(), position);
        assert_eq!(from_position.rotation(), Quat::IDENTITY);
        assert_eq!(from_position.scale(), Vec3::ONE);

        let from_rotation = Transform::from_rotation(rotation);

        assert_eq!(from_rotation.position(), Vec3::ZERO);
        assert_eq!(from_rotation.rotation(), rotation);
        assert_eq!(from_rotation.scale(), Vec3::ONE);

        let from_scale = Transform::from_scale(scale);

        assert_eq!(from_scale.position(), Vec3::ZERO);
        assert_eq!(from_scale.rotation(), Quat::IDENTITY);
        assert_eq!(from_scale.scale(), scale);

        let from_position_rotation = Transform::from_position_rotation(position, rotation);

        assert_eq!(from_position_rotation.position(), position);
        assert_eq!(from_position_rotation.rotation(), rotation);
        assert_eq!(from_position_rotation.scale(), Vec3::ONE);
    }

    #[test]
    fn setters_update_components() {
        let mut transform = Transform::default();

        transform.set_position(Vec3::new(1.0, 2.0, 3.0));
        transform.set_rotation(Quat::from_axis_angle(Vec3::UP, 45.0));
        transform.set_scale(Vec3::new(2.0, 3.0, 4.0));

        assert_eq!(transform.position(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.rotation(), Quat::from_axis_angle(Vec3::UP, 45.0));
        assert_eq!(transform.scale(), Vec3::new(2.0, 3.0, 4.0));

        let rotation = Quat::from_axis_angle(Vec3::RIGHT, 60.0);

        transform.set_position_and_rotation(Vec3::ZERO, rotation);

        assert_eq!(transform.position(), Vec3::ZERO);
        assert_eq!(transform.rotation(), rotation);
        assert_eq!(transform.scale(), Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn inverse_matrix_undoes_matrix() {
        let transform = Transform::new(
            Vec3::new(10.0, -3.0, 5.0),
            Quat::from_axis_angle(Vec3::UP, 30.0),
            Vec3::new(2.0, 0.5, 3.0),
        );

        let product = transform.matrix_from_scale_rotation_translation() * transform.unchecked_inverse_matrix();

        for (actual, expected) in product
            .to_array()
            .into_iter()
            .zip(Mat4::IDENTITY.to_array())
        {
            assert!((actual - expected).abs() < 1e-4, "{actual} != {expected}");
        }
    }

    #[test]
    fn matrix_path_matches_transform_point() {
        let transform = Transform::new(
            Vec3::new(10.0, -3.0, 5.0),
            Quat::from_axis_angle(Vec3::UP, 30.0),
            Vec3::new(2.0, 0.5, 3.0),
        );
        let point = Vec3::new(1.5, -2.0, 0.75);
        let via_quat = transform.transform_point(point);
        let via_matrix = transform.matrix_from_scale_rotation_translation().transform_point3(point);

        assert!((via_quat - via_matrix).length() < 1e-4);
    }

    #[test]
    fn transform_point_applies_scale_then_translation() {
        let position = Vec3::new(10.0, 0.0, 5.0);
        let scale = Vec3::new(2.0, 0.5, 3.0);
        let transform = Transform::new(position, Quat::IDENTITY, scale);
        let point = Vec3::new(1.0, 2.0, -1.0);

        assert_eq!(transform.transform_point(point), position + point * scale);
        assert_eq!(transform.transform_point(Vec3::ZERO), position);
    }

    #[test]
    fn translate_global_applies_translation_as_is() {
        let translation = Vec3::new(1.0, 2.0, 3.0);
        let mut transform = Transform::from_rotation(Quat::from_axis_angle(Vec3::UP, 90.0));

        transform.translate(translation, Space::Global);

        assert_eq!(transform.position(), translation);
    }

    #[test]
    fn translate_local_rotates_translation() {
        let rotation = Quat::from_axis_angle(Vec3::UP, 90.0);
        let translation = Vec3::RIGHT;
        let mut transform = Transform::from_rotation(rotation);

        transform.translate(translation, Space::Local);

        assert_eq!(transform.position(), rotation * translation);
        assert_ne!(transform.position(), translation);
    }

    #[test]
    fn rotate_global_applies_before_current_rotation() {
        let current = Quat::from_axis_angle(Vec3::UP, 90.0);
        let axis = Vec3::RIGHT;
        let angle = 45.0;
        let mut transform = Transform::from_rotation(current);

        transform.rotate(axis, angle, Space::Global);

        let delta = Quat::from_axis_angle(axis, angle);

        assert_eq!(transform.rotation(), delta * current);
        assert_ne!(transform.rotation(), current * delta);
    }

    #[test]
    fn rotate_local_applies_after_current_rotation() {
        let current = Quat::from_axis_angle(Vec3::UP, 90.0);
        let axis = Vec3::RIGHT;
        let angle = 45.0;
        let mut transform = Transform::from_rotation(current);

        transform.rotate(axis, angle, Space::Local);

        let delta = Quat::from_axis_angle(axis, angle);

        assert_eq!(transform.rotation(), current * delta);
        assert_ne!(transform.rotation(), delta * current);
    }

    #[test]
    fn directions_follow_rotation() {
        let mut transform = Transform::default();

        assert_eq!(transform.forward(), Vec3::FORWARD);
        assert_eq!(transform.backward(), Vec3::BACKWARD);
        assert_eq!(transform.right(), Vec3::RIGHT);
        assert_eq!(transform.left(), Vec3::LEFT);
        assert_eq!(transform.up(), Vec3::UP);
        assert_eq!(transform.down(), Vec3::DOWN);

        let rotation = Quat::from_axis_angle(Vec3::UP, 90.0);

        transform.set_rotation(rotation);

        assert_eq!(transform.forward(), rotation * Vec3::FORWARD);
        assert_eq!(transform.backward(), rotation * Vec3::BACKWARD);
        assert_eq!(transform.right(), rotation * Vec3::RIGHT);
        assert_eq!(transform.left(), rotation * Vec3::LEFT);
        assert_eq!(transform.up(), rotation * Vec3::UP);
        assert_eq!(transform.down(), rotation * Vec3::DOWN);
        assert_ne!(transform.forward(), Vec3::FORWARD);
        assert_ne!(transform.backward(), Vec3::BACKWARD);
        assert_ne!(transform.left(), Vec3::LEFT);
    }

    #[test]
    fn try_new_accepts_valid_values_and_normalizes_rotation() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::from_axis_angle(Vec3::UP, 45.0);
        let scale = Vec3::new(2.0, 3.0, 4.0);

        let transform = Transform::try_new(position, rotation, scale).unwrap();

        assert_eq!(transform.position(), position);
        assert_eq!(transform.rotation(), rotation.normalize());
        assert_eq!(transform.scale(), scale);
    }

    #[test]
    fn try_new_rejects_invalid_values() {
        assert!(
            Transform::try_new(Vec3::new(f32::NAN, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE,).is_none()
        );

        assert!(
            Transform::try_new(
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::new(1.0, f32::INFINITY, 1.0),
            )
            .is_none()
        );

        assert!(Transform::try_new(Vec3::ZERO, Quat::ZERO, Vec3::ONE,).is_none());
    }

    #[test]
    fn try_convenience_constructors_validate_inputs() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::from_axis_angle(Vec3::UP, 45.0);
        let scale = Vec3::new(2.0, 3.0, 4.0);

        assert!(Transform::try_from_position(position).is_some());
        assert!(Transform::try_from_rotation(rotation).is_some());
        assert!(Transform::try_from_scale(scale).is_some());
        assert!(Transform::try_from_position_rotation(position, rotation).is_some());

        assert!(Transform::try_from_position(Vec3::splat(f32::NAN)).is_none());
        assert!(Transform::try_from_rotation(Quat::ZERO).is_none());
        assert!(Transform::try_from_scale(Vec3::splat(f32::INFINITY)).is_none());
        assert!(Transform::try_from_position_rotation(position, Quat::ZERO).is_none());
    }

    #[test]
    fn try_setters_accept_valid_values() {
        let mut transform = Transform::default();

        let position = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::from_axis_angle(Vec3::UP, 45.0);
        let scale = Vec3::new(2.0, 3.0, 4.0);

        assert!(transform.try_set_position(position));
        assert!(transform.try_set_rotation(rotation));
        assert!(transform.try_set_scale(scale));

        assert_eq!(transform.position(), position);
        assert_eq!(transform.rotation(), rotation.normalize());
        assert_eq!(transform.scale(), scale);

        let new_position = Vec3::new(4.0, 5.0, 6.0);
        let new_rotation = Quat::from_axis_angle(Vec3::RIGHT, 30.0);

        assert!(transform.try_set_position_and_rotation(new_position, new_rotation));

        assert_eq!(transform.position(), new_position);
        assert_eq!(transform.rotation(), new_rotation.normalize());
        assert_eq!(transform.scale(), scale);
    }

    #[test]
    fn try_setters_reject_invalid_values_without_mutating_transform() {
        let mut transform = Transform::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 3.0, 4.0),
        );
        let original = transform;

        assert!(!transform.try_set_position(Vec3::splat(f32::NAN)));
        assert_eq!(transform, original);

        assert!(!transform.try_set_rotation(Quat::ZERO));
        assert_eq!(transform, original);

        assert!(!transform.try_set_scale(Vec3::splat(f32::INFINITY)));
        assert_eq!(transform, original);

        assert!(!transform.try_set_position_and_rotation(Vec3::splat(f32::NAN), Quat::IDENTITY,));
        assert_eq!(transform, original);

        assert!(!transform.try_set_position_and_rotation(Vec3::ZERO, Quat::ZERO,));
        assert_eq!(transform, original);
    }

    #[test]
    fn try_transform_point_accepts_finite_values() {
        let transform = Transform::new(
            Vec3::new(10.0, 0.0, 5.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 3.0, 4.0),
        );
        let point = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(
            transform.try_transform_point(point),
            Some(transform.transform_point(point)),
        );
    }

    #[test]
    fn try_transform_point_rejects_invalid_input_and_result() {
        let transform = Transform::default();

        assert!(
            transform
                .try_transform_point(Vec3::new(f32::NAN, 0.0, 0.0))
                .is_none()
        );

        let overflowing = Transform::from_position(Vec3::splat(f32::MAX));

        assert!(
            overflowing
                .try_transform_point(Vec3::splat(f32::MAX))
                .is_none()
        );
    }

    #[test]
    fn try_inverse_matrix_rejects_singular_transform() {
        let transform = Transform::from_scale(Vec3::new(1.0, 0.0, 1.0));

        assert!(transform.try_inverse_matrix().is_none());
    }

    #[test]
    fn try_translate_global_updates_position() {
        let mut transform = Transform::default();
        let translation = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(transform.try_translate(translation, Space::Global), Ok(()),);
        assert_eq!(transform.position(), translation);
    }

    #[test]
    fn try_translate_local_rotates_translation() {
        let rotation = Quat::from_axis_angle(Vec3::UP, 90.0);
        let mut transform = Transform::from_rotation(rotation);

        assert_eq!(transform.try_translate(Vec3::RIGHT, Space::Local), Ok(()),);
        assert_eq!(transform.position(), rotation * Vec3::RIGHT);
    }

    #[test]
    fn try_translate_rejects_invalid_translation_without_mutating_transform() {
        let mut transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let original = transform;

        assert_eq!(
            transform.try_translate(Vec3::new(f32::NAN, 0.0, 0.0), Space::Global,),
            Err(TransformError::NonFiniteTranslation),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_translate_rejects_non_finite_result_without_mutating_transform() {
        let mut transform = Transform::from_position(Vec3::splat(f32::MAX));
        let original = transform;

        assert_eq!(
            transform.try_translate(Vec3::splat(f32::MAX), Space::Global),
            Err(TransformError::NonFinitePosition),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_translate_local_rejects_invalid_rotation_without_mutating_transform() {
        let mut transform = Transform::from_rotation(Quat::ZERO);
        let original = transform;

        assert_eq!(
            transform.try_translate(Vec3::RIGHT, Space::Local),
            Err(TransformError::InvalidRotation),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_rotate_global_and_local_succeed() {
        let axis = Vec3::RIGHT;
        let angle = 45.0;
        let current = Quat::from_axis_angle(Vec3::UP, 90.0);
        let delta = Quat::from_axis_angle(axis, angle);

        let mut global = Transform::from_rotation(current);
        assert_eq!(global.try_rotate(axis, angle, Space::Global), Ok(()));
        assert_eq!(global.rotation(), (delta * current).normalize());

        let mut local = Transform::from_rotation(current);
        assert_eq!(local.try_rotate(axis, angle, Space::Local), Ok(()));
        assert_eq!(local.rotation(), (current * delta).normalize());
    }

    #[test]
    fn try_rotate_rejects_invalid_axis_without_mutating_transform() {
        let mut transform = Transform::default();
        let original = transform;

        assert_eq!(
            transform.try_rotate(Vec3::ZERO, 45.0, Space::Global),
            Err(TransformError::ZeroAxis),
        );
        assert_eq!(transform, original);

        assert_eq!(
            transform.try_rotate(Vec3::new(f32::NAN, 0.0, 0.0), 45.0, Space::Global,),
            Err(TransformError::NonFiniteAxis),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_rotate_rejects_non_finite_angle_without_mutating_transform() {
        let mut transform = Transform::default();
        let original = transform;

        assert_eq!(
            transform.try_rotate(Vec3::UP, f32::INFINITY, Space::Global),
            Err(TransformError::NonFiniteAngle),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_rotate_rejects_invalid_current_rotation_without_mutating_transform() {
        let mut transform = Transform::from_rotation(Quat::ZERO);
        let original = transform;

        assert_eq!(
            transform.try_rotate(Vec3::UP, 45.0, Space::Global),
            Err(TransformError::InvalidRotation),
        );
        assert_eq!(transform, original);
    }

    #[test]
    fn try_directions_return_expected_vectors() {
        let transform = Transform::default();

        assert_eq!(transform.try_forward(), Some(Vec3::FORWARD));
        assert_eq!(transform.try_backward(), Some(Vec3::BACKWARD));
        assert_eq!(transform.try_right(), Some(Vec3::RIGHT));
        assert_eq!(transform.try_left(), Some(Vec3::LEFT));
        assert_eq!(transform.try_up(), Some(Vec3::UP));
        assert_eq!(transform.try_down(), Some(Vec3::DOWN));
    }

    #[test]
    fn try_directions_reject_invalid_rotation() {
        let transform = Transform::from_rotation(Quat::ZERO);

        assert!(transform.try_forward().is_none());
        assert!(transform.try_backward().is_none());
        assert!(transform.try_right().is_none());
        assert!(transform.try_left().is_none());
        assert!(transform.try_up().is_none());
        assert!(transform.try_down().is_none());
    }
}
