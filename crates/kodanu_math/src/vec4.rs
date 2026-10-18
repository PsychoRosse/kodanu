use crate::Vec3;

use {
    serde::{Deserialize, Serialize},
    std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A four-dimensional vector with `f32` components.
///
/// `Vec4` is used for homogeneous coordinates, matrix columns,
/// and other four-component quantities.
///
/// Arithmetic operators applied to two vectors perform component-wise
/// operations.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vec4 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
    /// The w component of the vector.
    pub w: f32,
}

impl Vec4 {
    /// The zero vector `(0, 0, 0, 0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    /// The one vector `(1, 1, 1, 1)`.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    /// The positive Y-axis direction `(0, 1, 0, 0)`.
    pub const UP: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    /// The negative Y-axis direction `(0, -1, 0, 0)`.
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    /// The negative X-axis direction `(-1, 0, 0, 0)`.
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    /// The positive X-axis direction `(1, 0, 0, 0)`.
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    /// The forward direction along the negative Z-axis `(0, 0, -1, 0)`.
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    /// The backward direction along the positive Z-axis `(0, 0, 1, 0)`.
    pub const BACKWARD: Self = Self::new(0.0, 0.0, 1.0, 0.0);
}

impl Vec4 {
    /// Creates a vector from its four components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Vec4 {
    /// Creates a vector whose components are all set to `value`.
    #[must_use]
    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    /// Returns `true` if every component is finite.
    ///
    /// A component is finite when it is neither `NaN` nor infinite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns the squared length of the vector.
    ///
    /// This is equivalent to `self.dot(self)` and avoids the square root
    /// required by [`Self::length`].
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns the Euclidean length of the vector.
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the Euclidean length while avoiding intermediate
    /// overflow and underflow for finite vectors.
    ///
    /// Returns `NaN` if any component is non-finite.
    /// Returns infinity if the actual length exceeds `f32::MAX`.
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

    /// Returns the dot product of two vectors.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Returns the normalized vector, or `None` if any component is not
    /// finite or the vector is zero.
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

    /// Normalizes the vector without checking whether its length is valid.
    ///
    /// Zero-length vectors and vectors with non-finite or overflowing
    /// lengths may produce non-finite or zero components.
    /// Use [`Self::try_normalize`] when the input may be invalid.
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Returns a vector containing the absolute value of each component.
    #[must_use]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    /// Returns a vector containing the minimum of each pair of components.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    /// Returns a vector containing the maximum of each pair of components.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }

    /// Returns the first three components as a [`Vec3`], discarding `w`.
    #[must_use]
    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Add for Vec4 {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Vec4 {
    type Output = Self;

    /// Subtracts one vector from another component-wise.
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// This is not a dot product. Use [`crate::Vec4::dot`] to calculate the dot
    /// product of two vectors.
    fn mul(self, other: Vec4) -> Self {
        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;

    /// Divides one vector by another component-wise.
    ///
    /// Each component of the result is divided by the corresponding
    /// component of the other vector.
    fn div(self, other: Vec4) -> Self {
        Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    /// Multiplies each component of the vector by a scalar.
    fn mul(self, scalar: f32) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    /// Divides each component of the vector by a scalar.
    fn div(self, scalar: f32) -> Self {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

impl Neg for Vec4 {
    type Output = Self;

    /// Negates each component of the vector.
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl AddAssign for Vec4 {
    /// Adds another vector to this vector in place.
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl SubAssign for Vec4 {
    /// Subtracts another vector from this vector in place.
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl MulAssign for Vec4 {
    /// Multiplies this vector by another vector in place.
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl MulAssign<f32> for Vec4 {
    /// Multiplies each component of this vector by a scalar in place.
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl DivAssign for Vec4 {
    /// Divides this vector by another vector in place.
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

impl DivAssign<f32> for Vec4 {
    /// Divides each component of this vector by a scalar in place.
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}
