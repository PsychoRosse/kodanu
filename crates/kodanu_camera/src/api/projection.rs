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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_perspective_with_documented_values() {
        let Projection::Perspective(projection) = Projection::default();

        assert_eq!(projection.fov(), 90.0);
        assert_eq!(projection.aspect(), 1.0);
        assert_eq!(projection.near(), 0.03);
        assert_eq!(projection.far(), 1000.0);
    }

    #[test]
    fn projection_matrix_matches_perspective_rh() {
        let projection = Projection::Perspective(PerspectiveProjection::new(60.0, 2.0, 0.1, 500.0));

        assert_eq!(
            projection.projection_matrix(),
            Mat4::perspective_rh(60.0, 2.0, 0.1, 500.0)
        );
    }
}
