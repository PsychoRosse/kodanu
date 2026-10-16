use crate::Vec3;

use {
    serde::{Deserialize, Serialize},
    std::ops::{Div, DivAssign, Mul, MulAssign, Neg},
};

/// A quaternion representing a 3D rotation.
///
/// The quaternion is stored in `(x, y, z, w)` order, where `(x, y, z)`
/// is the vector part and `w` is the scalar part.
///
/// Rotation constructors that accept angles use degrees.
/// Quaternion operations generally assume a valid, non-zero quaternion
/// when normalization or rotation is required.
///
/// # Examples
///
/// ```
/// use kodanu_math::{Quat, Vec3};
///
/// let rotation = Quat::from_rotation_y(90.0);
/// let direction = rotation * Vec3::FORWARD;
///
/// // FORWARD is (0, 0, -1); a +90 degree rotation around Y maps it to (-1, 0, 0).
/// assert!((direction.x + 1.0).abs() < 1e-5);
/// assert!(direction.y.abs() < 1e-5);
/// assert!(direction.z.abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Quat {
    /// The x component of the quaternion's vector part.
    pub x: f32,
    /// The y component of the quaternion's vector part.
    pub y: f32,
    /// The z component of the quaternion's vector part.
    pub z: f32,
    /// The scalar component of the quaternion.
    pub w: f32,
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    /// The identity rotation `(0, 0, 0, 1)`.
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    /// The zero rotation `(0, 0, 0, 0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

impl Quat {
    /// Creates a quaternion from its `(x, y, z, w)` components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Quat {
    /// Returns `true` if every component is finite.
    ///
    /// A component is finite when it is neither `NaN` nor infinite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns the quaternion components as an array in `[x, y, z, w]` order.
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part `(x, y, z)` of the quaternion.
    #[must_use]
    pub const fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the squared magnitude of the quaternion.
    ///
    /// This is equivalent to `self.dot(self)` and avoids the square root
    /// required by [`Self::length`].
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns the magnitude of the quaternion.
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the quaternion magnitude while avoiding intermediate
    /// overflow and underflow for finite components.
    ///
    /// Returns `NaN` if any component is non-finite.
    /// Returns infinity if the actual magnitude exceeds `f32::MAX`.
    #[must_use]
    pub fn length_stable(self) -> f32 {
        if !self.is_finite() {
            return f32::NAN;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return 0.0;
        }

        let scaled = self / scale;
        scale * scaled.length()
    }

    /// Returns the dot product of two quaternions.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Returns the normalized quaternion, or `None` if any component is not
    /// finite or the quaternion is zero.
    ///
    /// Scales the components before computing the length to avoid
    /// intermediate overflow and underflow.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return None;
        }

        let scaled = Self::new(
            self.x / scale,
            self.y / scale,
            self.z / scale,
            self.w / scale,
        );

        Some(scaled / scaled.length())
    }

    /// Normalizes the quaternion without checking whether its length is valid.
    ///
    /// Zero-length quaternions and quaternions with non-finite or overflowing
    /// lengths may produce non-finite or zero components.
    /// Use [`Self::try_normalize`] when the input may be invalid.
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Returns the conjugate of the quaternion.
    ///
    /// The vector part is negated while the scalar part remains unchanged.
    #[must_use]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns the inverse quaternion.
    ///
    /// # Panics
    ///
    /// Panics if the quaternion is zero, non-finite, or its inverse cannot
    /// be represented by finite `f32` components.
    pub fn inverse(self) -> Self {
        self.try_inverse()
            .expect("cannot invert a zero or invalid quaternion")
    }

    /// Returns the inverse quaternion.
    ///
    /// Returns `None` if the quaternion is zero, non-finite, or its inverse
    /// cannot be represented by finite `f32` components.
    pub fn try_inverse(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return None;
        }

        let scaled = Self::new(
            self.x / scale,
            self.y / scale,
            self.z / scale,
            self.w / scale,
        );

        let norm_squared = scaled.length_squared();

        let inverse = scaled.conjugate() / norm_squared / scale;

        inverse.is_finite().then_some(inverse)
    }

    /// Converts this quaternion to an axis and angle in degrees.
    ///
    /// The quaternion is normalized before conversion. The axis for a zero
    /// rotation is `Vec3::RIGHT`, and its angle is `0.0`.
    ///
    /// # Panics
    ///
    /// Panics if the quaternion is zero or non-finite.
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        let q = self
            .try_normalize()
            .expect("cannot convert an invalid quaternion to axis-angle");

        // q and -q represent the same rotation. Choose the hemisphere with
        // non-negative w to return an angle in [0, 180] degrees.
        let q = if q.w < 0.0 { -q } else { q };

        let sin_half = q.x.hypot(q.y).hypot(q.z);

        if sin_half == 0.0 {
            return (Vec3::RIGHT, 0.0);
        }

        let axis = Vec3::new(q.x, q.y, q.z) / sin_half;
        let angle = 2.0 * sin_half.atan2(q.w).to_degrees();

        (axis, angle)
    }

    /// Attempts to rotate a vector by this quaternion.
    ///
    /// The quaternion and vector must be finite, and the quaternion must be
    /// non-zero. Returns `None` if the resulting vector is non-finite.
    pub fn try_rotate_vec3(self, vector: Vec3) -> Option<Vec3> {
        if !vector.is_finite() {
            return None;
        }

        let q = self.try_normalize()?;

        let qv = Vec3::new(q.x, q.y, q.z);
        let t = qv.cross(vector) * 2.0;
        let result = vector + t * q.w + qv.cross(t);

        result.is_finite().then_some(result)
    }

    /// Rotates a vector by this quaternion.
    ///
    /// The quaternion must be finite and non-zero. Invalid input causes a panic.
    pub fn rotate_vec3(self, vector: Vec3) -> Vec3 {
        self.try_rotate_vec3(vector)
            .expect("cannot rotate a vector with an invalid quaternion or vector")
    }
}

impl Quat {
    /// Creates a quaternion from Euler angles in degrees.
    ///
    /// Uses the Hamilton product `qx * qy * qz`, where `qx`, `qy`, and `qz`
    /// are rotations around the X, Y, and Z axes, respectively.
    ///
    /// When rotating a vector, the Z rotation is applied first, then Y,
    /// then X. The returned quaternion is not guaranteed to be unit length
    /// for all floating-point inputs.
    #[must_use]
    pub fn from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
        let x = x_degrees.rem_euclid(360.0).to_radians();
        let y = y_degrees.rem_euclid(360.0).to_radians();
        let z = z_degrees.rem_euclid(360.0).to_radians();

        let (sin_x, cos_x) = (x * 0.5).sin_cos();
        let (sin_y, cos_y) = (y * 0.5).sin_cos();
        let (sin_z, cos_z) = (z * 0.5).sin_cos();

        Self::new(
            sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z + sin_x * sin_y * cos_z,
            cos_x * cos_y * cos_z - sin_x * sin_y * sin_z,
        )
    }

    /// Creates a unit quaternion from an axis and an angle in degrees.
    ///
    /// The axis must be finite and non-zero, and the angle must be finite.
    ///
    /// # Panics
    ///
    /// Panics if the axis or angle is invalid.
    pub fn from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        Self::try_from_axis_angle(axis, degrees)
            .expect("axis-angle requires a finite non-zero axis and finite angle")
    }

    /// Attempts to create a unit quaternion from an axis and an angle in degrees.
    ///
    /// The axis must be finite and non-zero. Non-finite angles are rejected.
    /// The angle is reduced modulo 360 degrees before conversion to radians,
    /// avoiding overflow during degree-to-radian conversion.
    pub fn try_from_axis_angle(axis: Vec3, degrees: f32) -> Option<Self> {
        if !degrees.is_finite() {
            return None;
        }

        let axis = axis.try_normalize()?;
        let reduced_degrees = degrees.rem_euclid(360.0);
        let half_angle = reduced_degrees.to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        let result = Self::new(axis.x * sin, axis.y * sin, axis.z * sin, cos);

        result.is_finite().then_some(result)
    }

    /// Creates a quaternion from a scaled axis.
    ///
    /// The vector direction defines the rotation axis and its length defines
    /// the angle in degrees. Vectors with length at most `f32::EPSILON`
    /// produce the identity quaternion.
    ///
    /// Returns the identity quaternion if the input is non-finite or the
    /// rotation cannot be represented safely.
    pub fn from_scaled_axis(scaled_axis: Vec3) -> Self {
        Self::try_from_scaled_axis(scaled_axis).unwrap_or(Self::IDENTITY)
    }

    /// Attempts to create a quaternion from a scaled axis.
    ///
    /// The vector direction defines the rotation axis and its length defines
    /// the angle in degrees. Returns `None` for non-finite input or if the
    /// resulting rotation cannot be represented safely.
    pub fn try_from_scaled_axis(scaled_axis: Vec3) -> Option<Self> {
        if !scaled_axis.is_finite() {
            return None;
        }

        let degrees = scaled_axis.length_stable();

        if !degrees.is_finite() {
            return None;
        }

        if degrees <= f32::EPSILON {
            return Some(Self::IDENTITY);
        }

        let axis = scaled_axis.try_normalize()?;
        Self::try_from_axis_angle(axis, degrees)
    }

    /// Creates a quaternion from `(x, y, z, w)` components.
    #[must_use]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(x, y, z, w)
    }

    /// Creates a quaternion from an array in `[x, y, z, w]` order.
    #[must_use]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    /// Creates a rotation around the X axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_x(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(sin, 0.0, 0.0, cos)
    }

    /// Creates a rotation around the Y axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_y(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, sin, 0.0, cos)
    }

    /// Creates a rotation around the Z axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_z(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, 0.0, sin, cos)
    }
}

impl Mul for Quat {
    type Output = Self;

    /// Multiplies two quaternions.
    ///
    /// The result composes the rotations: applying `rhs` first and then
    /// `self` is equivalent to applying the result.
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl MulAssign for Quat {
    /// Multiplies this quaternion by another in place.
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    /// Multiplies each component of the quaternion by a scalar.
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f32> for Quat {
    /// Multiplies each component of this quaternion by a scalar in place.
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    /// Divides each component of the quaternion by a scalar.
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}
impl DivAssign<f32> for Quat {
    /// Divides each component of this quaternion by a scalar in place.
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Neg for Quat {
    type Output = Self;

    /// Negates each component of the quaternion.
    ///
    /// The negated quaternion represents the same rotation.
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    /// Rotates a vector by this quaternion.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::rotate_vec3`].
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.rotate_vec3(rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EPSILON: f32 = 1.0e-5;

    fn approx_eq(a: f32, b: f32) {
        assert!(
            (a - b).abs() <= EPSILON,
            "expected {a} ≈ {b}, difference = {}",
            (a - b).abs()
        );
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3) {
        approx_eq(a.x, b.x);
        approx_eq(a.y, b.y);
        approx_eq(a.z, b.z);
    }

    #[test]
    fn to_axis_angle_handles_small_angles_and_negative_quaternions() {
        let q = Quat::from_axis_angle(Vec3::UP, 90.0);
        let (axis, angle) = (-q).to_axis_angle();

        vec3_approx_eq(axis, Vec3::UP);
        approx_eq(angle, 90.0);
    }

    #[test]
    fn from_euler_matches_hamilton_product_order() {
        let x = Quat::from_rotation_x(30.0);
        let y = Quat::from_rotation_y(45.0);
        let z = Quat::from_rotation_z(60.0);

        let expected = x * y * z;
        let actual = Quat::from_euler(30.0, 45.0, 60.0);

        approx_eq(actual.x, expected.x);
        approx_eq(actual.y, expected.y);
        approx_eq(actual.z, expected.z);
        approx_eq(actual.w, expected.w);
    }

    #[test]
    fn length_stable_handles_identity_and_scalar_component() {
        assert_eq!(Quat::IDENTITY.length_stable(), 1.0);
        assert_eq!(Quat::new(0.0, 0.0, 0.0, 2.0).length_stable(), 2.0);
    }

    #[test]
    fn length_stable_handles_large_quaternions() {
        let q = Quat::new(1e30, 1e30, 1e30, 1e30);
        let length = q.length_stable();

        assert!(length.is_finite());
        assert!((length / 1e30 - 2.0).abs() < 1e-6);
    }

    #[test]
    fn length_stable_returns_nan_for_non_finite_quaternions() {
        assert!(Quat::new(f32::NAN, 0.0, 0.0, 1.0).length_stable().is_nan());
        assert!(
            Quat::new(0.0, 0.0, 0.0, f32::INFINITY)
                .length_stable()
                .is_nan()
        );
    }

    #[test]
    fn try_inverse_handles_large_quaternions() {
        let q = Quat::new(1.0e30, -2.0e30, 3.0e30, 4.0e30);

        let inverse = q.try_inverse().expect("inverse should be finite");
        let product = q * inverse;

        approx_eq(product.x, 0.0);
        approx_eq(product.y, 0.0);
        approx_eq(product.z, 0.0);
        approx_eq(product.w, 1.0);
    }

    #[test]
    fn try_inverse_handles_identity() {
        assert_eq!(Quat::IDENTITY.try_inverse(), Some(Quat::IDENTITY));
    }

    #[test]
    fn try_inverse_rejects_zero_and_non_finite_quaternions() {
        let zero = Quat::new(0.0, 0.0, 0.0, 0.0);

        assert!(zero.try_inverse().is_none());
        assert!(Quat::new(f32::NAN, 0.0, 0.0, 1.0).try_inverse().is_none());
        assert!(
            Quat::new(f32::INFINITY, 0.0, 0.0, 1.0)
                .try_inverse()
                .is_none()
        );
    }

    #[test]
    fn try_rotate_vec3_rotates_with_identity() {
        assert_eq!(
            Quat::IDENTITY.try_rotate_vec3(Vec3::RIGHT),
            Some(Vec3::RIGHT)
        );
    }

    #[test]
    fn try_rotate_vec3_rejects_invalid_inputs() {
        let zero = Quat::new(0.0, 0.0, 0.0, 0.0);

        assert!(zero.try_rotate_vec3(Vec3::RIGHT).is_none());

        assert!(
            Quat::IDENTITY
                .try_rotate_vec3(Vec3::new(f32::NAN, 0.0, 0.0))
                .is_none()
        );

        assert!(
            Quat::IDENTITY
                .try_rotate_vec3(Vec3::new(f32::INFINITY, 0.0, 0.0))
                .is_none()
        );
    }

    #[test]
    fn rotate_vec3_normalizes_non_unit_quaternions() {
        let q = Quat::from_rotation_y(90.0) * 3.0;

        let rotated = q
            .try_rotate_vec3(Vec3::FORWARD)
            .expect("rotation should succeed");

        vec3_approx_eq(rotated, Vec3::LEFT);
    }

    #[test]
    fn to_axis_angle_handles_negative_quaternions() {
        let q = Quat::from_axis_angle(Vec3::UP, 90.0);

        let (axis_a, angle_a) = q.to_axis_angle();
        let (axis_b, angle_b) = (-q).to_axis_angle();

        vec3_approx_eq(axis_a, Vec3::UP);
        vec3_approx_eq(axis_b, axis_a);
        approx_eq(angle_a, 90.0);
        approx_eq(angle_b, angle_a);
    }

    #[test]
    fn to_axis_angle_preserves_small_nonzero_angles() {
        let q = Quat::from_axis_angle(Vec3::UP, 0.001);
        let (axis, angle) = q.to_axis_angle();

        vec3_approx_eq(axis, Vec3::UP);
        assert!(angle > 0.0);
        approx_eq(angle, 0.001);
    }

    #[test]
    fn to_axis_angle_returns_canonical_zero_rotation() {
        let (axis, angle) = Quat::IDENTITY.to_axis_angle();

        assert_eq!(axis, Vec3::RIGHT);
        assert_eq!(angle, 0.0);
    }

    #[test]
    fn try_from_axis_angle_handles_large_finite_angles() {
        let result = Quat::try_from_axis_angle(Vec3::UP, f32::MAX);

        assert!(result.is_some());
        assert!(result.unwrap().is_finite());
    }

    #[test]
    fn try_from_scaled_axis_rejects_non_finite_and_overflowing_lengths() {
        assert!(Quat::try_from_scaled_axis(Vec3::new(f32::INFINITY, 0.0, 0.0)).is_none());

        assert!(Quat::try_from_scaled_axis(Vec3::new(f32::MAX, f32::MAX, 0.0)).is_none());

        assert_eq!(Quat::try_from_scaled_axis(Vec3::ZERO), Some(Quat::IDENTITY));
    }
}
