use {
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

/// Errors returned by the fallible operations of [`crate::Transform`].
///
/// [`crate::Transform::try_translate`] and [`crate::Transform::try_rotate`]
/// validate their input and report the reason an operation was rejected
/// through this type instead of modifying the transform.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TransformError {
    /// The translation contains non-finite values.
    #[error("Translation contains non-finite values")]
    NonFiniteTranslation,
    /// The resulting position contains non-finite values.
    #[error("Rotation axis contains non-finite values")]
    NonFinitePosition,
    /// The rotation axis contains non-finite values.
    #[error("Translation contains non-finite values")]
    NonFiniteAxis,
    /// The rotation axis is zero.
    #[error("Rotation axis must be non-zero")]
    ZeroAxis,
    /// The rotation angle is not finite.
    #[error("Rotation angle is not finite")]
    NonFiniteAngle,
    /// The rotation quaternion is not finite or cannot be normalized.
    #[error("Rotation quaternion is invalid")]
    InvalidRotation,
}
