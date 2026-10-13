/// Selects the space in which a transformation is applied.
///
/// World-space operations are unaffected by the transform's rotation,
/// local-space operations are rotated by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Space {
    /// The world space.
    Global,
    /// The space defined by the transform's current rotation.
    Local,
}
