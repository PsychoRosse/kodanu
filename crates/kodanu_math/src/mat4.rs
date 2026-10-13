use crate::{Quat, Vec3, Vec4};

use {
    serde::{Deserialize, Serialize},
    std::ops::{Mul, MulAssign},
};

/// A 4×4 matrix with `f32` components.
///
/// The matrix is stored as four column vectors: [`Self::x_axis`],
/// [`Self::y_axis`], [`Self::z_axis`], and [`Self::w_axis`].
///
/// Matrix-vector multiplication uses column vectors. Translation is stored
/// in the XYZ components of `w_axis`.
///
/// The matrix supports common 3D transformations, including translation,
/// scaling, rotation, perspective projection, and point or vector
/// transformation.
///
/// # Examples
///
/// ```
/// use kodanu_math::{Mat4, Vec3};
///
/// let translation = Mat4::from_translation(Vec3::new(2.0, 3.0, 4.0));
/// let point = translation.transform_point3(Vec3::ZERO);
///
/// assert_eq!(point, Vec3::new(2.0, 3.0, 4.0));
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mat4 {
    /// The first column of the matrix.
    pub x_axis: Vec4,
    /// The second column of the matrix.
    pub y_axis: Vec4,
    /// The third column of the matrix.
    pub z_axis: Vec4,
    /// The fourth column of the matrix.
    ///
    /// For affine transformation matrices, its XYZ components store translation.
    pub w_axis: Vec4,
}

impl Mat4 {
    /// The 4×4 identity matrix.
    pub const IDENTITY: Self = Self::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
}

impl Mat4 {
    /// Creates a matrix from its four column vectors.
    #[must_use]
    pub const fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }
}

impl Mat4 {
    /// Returns `true` if every component is finite.
    ///
    /// A component is finite when it is neither `NaN` nor infinite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    /// Creates a matrix from four column vectors.
    ///
    /// This is equivalent to [`Self::new`].
    #[must_use]
    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self::new(x_axis, y_axis, z_axis, w_axis)
    }

    /// Creates a matrix from 16 values in column-major order.
    ///
    /// The first four values form the first column, the next four form
    /// the second column, and so on.
    #[must_use]
    pub const fn from_array(array: [f32; 16]) -> Self {
        Self::new(
            Vec4::new(array[0], array[1], array[2], array[3]),
            Vec4::new(array[4], array[5], array[6], array[7]),
            Vec4::new(array[8], array[9], array[10], array[11]),
            Vec4::new(array[12], array[13], array[14], array[15]),
        )
    }

    /// Creates a translation matrix.
    #[must_use]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    /// Creates a scale matrix.
    ///
    /// The scale factors are applied independently along the X, Y, and Z axes.
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(
            Vec4::new(scale.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Creates a rotation matrix from a quaternion.
    ///
    /// The quaternion is normalized before conversion.
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Self {
        let q = rotation.normalize();

        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;

        let xx = q.x * x2;
        let xy = q.x * y2;
        let xz = q.x * z2;

        let yy = q.y * y2;
        let yz = q.y * z2;
        let zz = q.z * z2;

        let wx = q.w * x2;
        let wy = q.w * y2;
        let wz = q.w * z2;

        Self::new(
            Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
            Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
            Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Creates a transformation matrix from scale, rotation, and translation.
    ///
    /// The resulting matrix applies scale and rotation to points before
    /// applying translation, using the matrix convention implemented by
    /// this type.
    #[must_use]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let rotation = Self::from_quat(rotation);

        Self::new(
            rotation.x_axis * scale.x,
            rotation.y_axis * scale.y,
            rotation.z_axis * scale.z,
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    /// Converts the matrix to 16 values in column-major order.
    #[must_use]
    pub const fn to_array(&self) -> [f32; 16] {
        [
            self.x_axis.x,
            self.x_axis.y,
            self.x_axis.z,
            self.x_axis.w,
            self.y_axis.x,
            self.y_axis.y,
            self.y_axis.z,
            self.y_axis.w,
            self.z_axis.x,
            self.z_axis.y,
            self.z_axis.z,
            self.z_axis.w,
            self.w_axis.x,
            self.w_axis.y,
            self.w_axis.z,
            self.w_axis.w,
        ]
    }

    /// Converts the matrix to four columns, each represented as an array.
    ///
    /// The returned layout is `[[x_axis], [y_axis], [z_axis], [w_axis]]`.
    #[must_use]
    pub const fn to_cols_array_2d(self) -> [[f32; 4]; 4] {
        [
            [self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w],
            [self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w],
            [self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w],
            [self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w],
        ]
    }

    /// Transforms a point using this matrix.
    ///
    /// The point is treated as a homogeneous vector with `w = 1`.
    /// If the resulting `w` component is non-zero, the XYZ components
    /// are divided by `w`.
    #[must_use]
    pub fn transform_point3(self, point: Vec3) -> Vec3 {
        let result = self * Vec4::new(point.x, point.y, point.z, 1.0);

        if result.w != 0.0 {
            Vec3::new(
                result.x / result.w,
                result.y / result.w,
                result.z / result.w,
            )
        } else {
            Vec3::new(result.x, result.y, result.z)
        }
    }

    /// Transforms a point using this matrix.
    ///
    /// The point is treated as a homogeneous vector with `w = 1`, and the
    /// XYZ components of the result are divided by `w`.
    ///
    /// Returns `None` if the point or this matrix is not finite, the
    /// resulting `w` component is zero, or the resulting point is not finite.
    #[must_use]
    pub fn try_transform_point3(self, point: Vec3) -> Option<Vec3> {
        if !point.is_finite() || !self.is_finite() {
            return None;
        }

        let result = self * Vec4::new(point.x, point.y, point.z, 1.0);

        if !result.is_finite() || result.w == 0.0 {
            return None;
        }

        let point = Vec3::new(
            result.x / result.w,
            result.y / result.w,
            result.z / result.w,
        );

        point.is_finite().then_some(point)
    }

    /// Transforms a direction vector using this matrix.
    ///
    /// The vector is treated as a homogeneous vector with `w = 0`,
    /// so translation does not affect the result.
    #[must_use]
    pub fn transform_vector3(self, vector: Vec3) -> Vec3 {
        let result = self * Vec4::new(vector.x, vector.y, vector.z, 0.0);

        Vec3::new(result.x, result.y, result.z)
    }

    /// Returns the transpose of this matrix.
    ///
    /// Rows and columns are exchanged.
    #[must_use]
    pub const fn transpose(self) -> Self {
        Self::new(
            Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }

    /// Returns the determinant of the matrix.
    #[must_use]
    pub fn determinant(self) -> f32 {
        let m = self.to_array();

        let a0 = m[0] * m[5] - m[1] * m[4];
        let a1 = m[0] * m[6] - m[2] * m[4];
        let a2 = m[0] * m[7] - m[3] * m[4];
        let a3 = m[1] * m[6] - m[2] * m[5];
        let a4 = m[1] * m[7] - m[3] * m[5];
        let a5 = m[2] * m[7] - m[3] * m[6];

        let b0 = m[8] * m[13] - m[9] * m[12];
        let b1 = m[8] * m[14] - m[10] * m[12];
        let b2 = m[8] * m[15] - m[11] * m[12];
        let b3 = m[9] * m[14] - m[10] * m[13];
        let b4 = m[9] * m[15] - m[11] * m[13];
        let b5 = m[10] * m[15] - m[11] * m[14];

        a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0
    }

    /// Returns the inverse of this matrix.
    ///
    /// If the determinant's absolute value is less than or equal to
    /// `f32::EPSILON`, this method returns [`Self::IDENTITY`] instead.
    ///
    /// This fallback does not indicate that the matrix was inverted
    /// successfully. Check [`Self::determinant`] if singular matrices
    /// are possible.
    #[must_use]
    pub fn inverse(self) -> Self {
        let m = self.to_array();

        let a0 = m[0] * m[5] - m[1] * m[4];
        let a1 = m[0] * m[6] - m[2] * m[4];
        let a2 = m[0] * m[7] - m[3] * m[4];
        let a3 = m[1] * m[6] - m[2] * m[5];
        let a4 = m[1] * m[7] - m[3] * m[5];
        let a5 = m[2] * m[7] - m[3] * m[6];

        let b0 = m[8] * m[13] - m[9] * m[12];
        let b1 = m[8] * m[14] - m[10] * m[12];
        let b2 = m[8] * m[15] - m[11] * m[12];
        let b3 = m[9] * m[14] - m[10] * m[13];
        let b4 = m[9] * m[15] - m[11] * m[13];
        let b5 = m[10] * m[15] - m[11] * m[14];

        let determinant = a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0;

        if determinant.abs() <= f32::EPSILON {
            return Self::IDENTITY;
        }

        let inv_det = 1.0 / determinant;

        Self::new(
            Vec4::new(
                (m[5] * b5 - m[6] * b4 + m[7] * b3) * inv_det,
                (-m[1] * b5 + m[2] * b4 - m[3] * b3) * inv_det,
                (m[13] * a5 - m[14] * a4 + m[15] * a3) * inv_det,
                (-m[9] * a5 + m[10] * a4 - m[11] * a3) * inv_det,
            ),
            Vec4::new(
                (-m[4] * b5 + m[6] * b2 - m[7] * b1) * inv_det,
                (m[0] * b5 - m[2] * b2 + m[3] * b1) * inv_det,
                (-m[12] * a5 + m[14] * a2 - m[15] * a1) * inv_det,
                (m[8] * a5 - m[10] * a2 + m[11] * a1) * inv_det,
            ),
            Vec4::new(
                (m[4] * b4 - m[5] * b2 + m[7] * b0) * inv_det,
                (-m[0] * b4 + m[1] * b2 - m[3] * b0) * inv_det,
                (m[12] * a4 - m[13] * a2 + m[15] * a0) * inv_det,
                (-m[8] * a4 + m[9] * a2 - m[11] * a0) * inv_det,
            ),
            Vec4::new(
                (-m[4] * b3 + m[5] * b1 - m[6] * b0) * inv_det,
                (m[0] * b3 - m[1] * b1 + m[2] * b0) * inv_det,
                (-m[12] * a3 + m[13] * a1 - m[14] * a0) * inv_det,
                (m[8] * a3 - m[9] * a1 + m[10] * a0) * inv_det,
            ),
        )
    }

    /// Returns the inverse of this matrix, or `None` if the determinant
    /// is not finite or its absolute value is less than or equal to
    /// `f32::EPSILON`.
    #[must_use]
    pub fn try_inverse(self) -> Option<Self> {
        let determinant = self.determinant();

        if !determinant.is_finite() || determinant.abs() <= f32::EPSILON {
            return None;
        }

        let inverse = self.inverse();

        inverse.is_finite().then_some(inverse)
    }

    /// Creates a right-handed perspective projection matrix.
    ///
    /// `fov_y_degrees` is the vertical field of view in degrees.
    /// `aspect_ratio` is the viewport width divided by its height.
    /// `near` and `far` specify the near and far clipping-plane distances.
    ///
    /// The method does not validate its arguments. The field of view,
    /// aspect ratio, and clipping-plane distances must form a valid
    /// perspective projection.
    #[must_use]
    pub fn perspective_rh(fov_y_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y_degrees.to_radians() * 0.5).tan();

        Self::new(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / (near - far), -1.0),
            Vec4::new(0.0, 0.0, (near * far) / (near - far), 0.0),
        )
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x_axis.x * rhs.x
                + self.y_axis.x * rhs.y
                + self.z_axis.x * rhs.z
                + self.w_axis.x * rhs.w,
            self.x_axis.y * rhs.x
                + self.y_axis.y * rhs.y
                + self.z_axis.y * rhs.z
                + self.w_axis.y * rhs.w,
            self.x_axis.z * rhs.x
                + self.y_axis.z * rhs.y
                + self.z_axis.z * rhs.z
                + self.w_axis.z * rhs.w,
            self.x_axis.w * rhs.x
                + self.y_axis.w * rhs.y
                + self.z_axis.w * rhs.z
                + self.w_axis.w * rhs.w,
        )
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_inverse_inverts_invertible_matrix() {
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(2.0, 0.5, 3.0),
            Quat::from_axis_angle(Vec3::UP, 30.0),
            Vec3::new(10.0, -3.0, 5.0),
        );
        let inverse = matrix.try_inverse();

        assert!(inverse.is_some());

        let product = matrix * inverse.unwrap_or(Mat4::IDENTITY);

        for (actual, expected) in product
            .to_array()
            .into_iter()
            .zip(Mat4::IDENTITY.to_array())
        {
            assert!((actual - expected).abs() < 1e-4, "{actual} != {expected}");
        }
    }

    #[test]
    fn try_inverse_rejects_singular_matrix() {
        assert_eq!(Mat4::default().try_inverse(), None);
    }

    #[test]
    fn try_inverse_rejects_non_finite_determinant() {
        let matrix = Mat4::from_scale(Vec3::new(f32::INFINITY, 1.0, 1.0));

        assert_eq!(matrix.try_inverse(), None);
    }

    #[test]
    fn try_inverse_rejects_zero_scale_axis() {
        let matrix = Mat4::from_scale(Vec3::new(1.0, 0.0, 1.0));

        assert_eq!(matrix.try_inverse(), None);
    }
}
