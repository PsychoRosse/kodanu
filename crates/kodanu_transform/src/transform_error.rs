use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

/// Errors returned by the fallible operations of [`crate::Transform`].
///
/// [`crate::Transform::try_translate`] and [`crate::Transform::try_rotate`]
/// validate their input and report the reason an operation was rejected
/// through this type instead of modifying the transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransformError {
    /// The translation contains non-finite values.
    NonFiniteTranslation,
    /// The resulting position contains non-finite values.
    NonFinitePosition,
    /// The rotation axis contains non-finite values.
    NonFiniteAxis,
    /// The rotation axis is zero.
    ZeroAxis,
    /// The rotation angle is not finite.
    NonFiniteAngle,
    /// The rotation quaternion is not finite or cannot be normalized.
    InvalidRotation,
}

impl Display for TransformError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::NonFiniteTranslation => {
                write!(f, "translation contains non-finite values")
            }
            Self::NonFinitePosition => {
                write!(f, "resulting position contains non-finite values")
            }
            Self::NonFiniteAxis => {
                write!(f, "rotation axis contains non-finite values")
            }
            Self::ZeroAxis => {
                write!(f, "rotation axis must be non-zero")
            }
            Self::NonFiniteAngle => {
                write!(f, "rotation angle is not finite")
            }
            Self::InvalidRotation => {
                write!(f, "rotation quaternion is invalid")
            }
        }
    }
}

impl Error for TransformError {}
