use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransformError {
    #[error("Translation contains non-finite values")]
    NonFiniteTranslation,
    #[error("Rotation axis contains non-finite values")]
    NonFinitePosition,
    #[error("Translation contains non-finite values")]
    NonFiniteAxis,
    #[error("Rotation axis must be non-zero")]
    ZeroAxis,
    #[error("Rotation angle is not finite")]
    NonFiniteAngle,
    #[error("Rotation quaternion is invalid")]
    InvalidRotation,
}
