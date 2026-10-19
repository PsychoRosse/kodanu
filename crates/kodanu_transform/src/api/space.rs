use serde::{Deserialize, Serialize};

/// Selects the coordinate space in which a transformation is applied.
///
/// `Global` applies operations in world space, while `Local` applies
/// operations relative to the transform's current orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Space {
    /// The world space.
    Global,
    /// The space defined by the transform's current rotation.
    Local,
}
