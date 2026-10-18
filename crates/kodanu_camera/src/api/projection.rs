use crate::PerspectiveProjection;

use kodanu_math::Mat4;

/// Describes the projection used by a [`crate::Camera`].
///
/// A projection converts view-space coordinates into clip space.
#[derive(Debug, Clone, Copy)]
pub enum Projection {
    /// A perspective projection.
    Perspective(PerspectiveProjection),
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

impl Projection {
    /// Builds the projection matrix.
    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        match self {
            Projection::Perspective(projection) => projection.projection_matrix(),
        }
    }
}
