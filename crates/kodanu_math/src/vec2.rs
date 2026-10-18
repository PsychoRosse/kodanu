use {
    serde::{Deserialize, Serialize},
    std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A two-dimensional vector with `f32` components.
///
/// `Vec2` is commonly used for 2D positions, directions, and scales.
/// Its components are public and can be accessed or modified directly.
///
/// Arithmetic operators perform component-wise operations when applied
/// to another vector.
///
/// # Examples
///
/// ```
/// use kodanu_math::Vec2;
///
/// let position = Vec2::new(10.0, 5.0);
/// let offset = Vec2::new(2.0, 3.0);
///
/// let result = position + offset;
///
/// assert_eq!(result, Vec2::new(12.0, 8.0));
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vec2 {
    /// The horizontal component of the vector.
    pub x: f32,
    /// The vertical component of the vector.
    pub y: f32,
}

impl Vec2 {
    /// The zero vector `(0, 0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0);
    /// The one vector `(1, 1)`.
    pub const ONE: Self = Self::new(1.0, 1.0);
    /// The positive Y-axis direction `(0, 1)`.
    pub const UP: Self = Self::new(0.0, 1.0);
    /// The negative Y-axis direction `(0, -1)`.
    pub const DOWN: Self = Self::new(0.0, -1.0);
    /// The negative X-axis direction `(-1, 0)`.
    pub const LEFT: Self = Self::new(-1.0, 0.0);
    /// The positive X-axis direction `(1, 0)`.
    pub const RIGHT: Self = Self::new(1.0, 0.0);
}

impl Vec2 {
    /// Creates a vector from its X and Y components.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vec2 {
    /// Returns `true` if every component is finite.
    ///
    /// A component is finite when it is neither `NaN` nor infinite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns the squared length of the vector.
    ///
    /// This is equivalent to `self.dot(self)` and avoids the square root
    /// required by [`Self::length`].
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

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return 0.0;
        }

        let scaled = self / scale;
        scale * scaled.length()
    }

    /// Returns the dot product of two vectors.
    ///
    /// The result is `self.x * other.x + self.y * other.y`.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the normalized vector, or `None` if any component is not
    /// finite or the vector is zero.
    ///
    /// Normalization produces a vector with length approximately equal to `1`.
    /// The vector is scaled by its largest component before dividing, so
    /// finite vectors of any magnitude normalize without overflowing.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return None;
        }

        let scaled = Self::new(self.x / scale, self.y / scale);
        let length = scaled.length();

        Some(scaled / length)
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
}

impl Add for Vec2 {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    /// Subtracts one vector from another component-wise.
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// Each component of the result is the product of the corresponding
    /// components of the input vectors.
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Div for Vec2 {
    type Output = Self;

    /// Divides one vector by another component-wise.
    ///
    /// Each component of the result is divided by the corresponding
    /// component of the other vector.
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    /// Multiplies each component of the vector by a scalar.
    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    /// Divides each component of the vector by a scalar.
    fn div(self, other: f32) -> Self {
        Self::new(self.x / other, self.y / other)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    /// Negates each component of the vector.
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    /// Adds another vector to this vector in place.
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    /// Subtracts another vector from this vector in place.
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    /// Multiplies each component of this vector by a scalar in place.
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl DivAssign<f32> for Vec2 {
    /// Divides each component of this vector by a scalar in place.
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}
