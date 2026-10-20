use {
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
