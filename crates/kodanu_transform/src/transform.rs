use crate::Space;

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Quat, Vec3},
    serde::{Deserialize, Serialize},
};

/// Describes the position, rotation, and scale of an entity in 3D space.
///
/// `Transform` uses a translation, rotation, and scale representation.
///
/// # Coordinate system
///
/// The coordinate system follows the conventions defined by [`Vec3`].
///
/// # Examples
///
/// ```
/// use {kodanu_math::*, kodanu_transform::*};
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
    ///
    /// # Examples
    ///
    /// ```
    /// use {kodanu_math::*, kodanu_transform::*};
    ///
    /// let transform = Transform::try_new(Vec3::ZERO, Quat::from_scaled_axis(Vec3::ZERO), Vec3::ONE);
    ///
    /// assert_eq!(transform, Some(Transform::default()));
    ///
    /// assert!(Transform::try_new(Vec3::new(f32::NAN, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE).is_none());
    /// ```
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
    #[must_use]
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Returns the inverse transformation matrix.
    ///
    /// This is useful for transforming world-space coordinates into the
    /// local space represented by this transform.
    #[must_use]
    pub fn uncheked_inverse_matrix(&self) -> Mat4 {
        self.matrix().inverse()
    }

    /// Returns the inverse transformation matrix, or `None` if the matrix is not invertible.
    ///
    /// This is useful for transforming world-space coordinates into the local space
    /// represented by this transform.
    ///
    /// Returns `None` if the transformation matrix is singular.
    #[must_use]
    pub fn try_inverse_matrix(&self) -> Option<Mat4> {
        self.matrix().try_inverse()
    }

    /// Transforms a point from local space into world space.
    ///
    /// The point is scaled, rotated, and then translated.
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    /// Sets the world-space position.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Sets the rotation.
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    /// Sets the position and rotation.
    ///
    /// The current scale is preserved.
    pub fn set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    /// Sets the scale.
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    /// Returns the world-space position.
    #[must_use]
    pub const fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the rotation.
    #[must_use]
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Returns the scale.
    #[must_use]
    pub const fn scale(&self) -> Vec3 {
        self.scale
    }
}

impl Transform {
    /// Returns the forward direction of the transform in world space.
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::FORWARD
    }

    /// Returns the backward direction of the transform in world space.
    #[must_use]
    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::BACKWARD
    }

    /// Returns the right direction of the transform in world space.
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::RIGHT
    }

    /// Returns the left direction of the transform in world space.
    #[must_use]
    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::LEFT
    }

    /// Returns the up direction of the transform in world space.
    #[must_use]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::UP
    }

    /// Returns the down direction of the transform in world space.
    #[must_use]
    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::DOWN
    }
}

impl Transform {
    /// Moves the transform in world or local space.
    ///
    /// With [`crate::Space::Global`] the translation is applied to the position as-is.
    /// With [`crate::Space::Local`] the translation is rotated by the transform's
    /// current rotation before being applied.
    pub fn translate(&mut self, translation: Vec3, space: Space) {
        match space {
            Space::Global => self.position += translation,
            Space::Local => self.position += self.rotation * translation,
        }
    }

    /// Rotates the transform around an axis in world or local space.
    ///
    /// With [`crate::Space::Global`] the axis lies in world space and the rotation
    /// is applied before the current rotation. With [`crate::Space::Local`] the axis
    /// lies in the transform's local space and the rotation is applied after it.
    ///
    /// `axis` is normalized before constructing the rotation.
    ///
    /// # Panics
    ///
    /// Panics if `axis` is zero, non-finite, or `angle` is not finite.
    /// See [`Quat::from_axis_angle`].
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
    /// assert!((transform.forward() - Vec3::LEFT).length() < 1e-5);
    /// ```
    pub fn rotate(&mut self, axis: Vec3, angle: f32, space: Space) {
        match space {
            Space::Global => {
                self.rotation = Quat::from_axis_angle(axis.normalize(), angle) * self.rotation
            }
            Space::Local => self.rotation *= Quat::from_axis_angle(axis.normalize(), angle),
        }
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

    /// Creates a transform with the given rotation.
    ///
    /// Position is [`Vec3::ZERO`] and scale is [`Vec3::ONE`].
    #[must_use]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    /// Creates a transform with the given scale.
    ///
    /// Position is [`Vec3::ZERO`] and rotation is [`Quat::IDENTITY`].
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    /// Creates a transform with the given position and rotation.
    ///
    /// Scale is initialized to [`Vec3::ONE`].
    #[must_use]
    pub const fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
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

        let product = transform.matrix() * transform.uncheked_inverse_matrix();

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
        let via_matrix = transform.matrix().transform_point3(point);

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
}
