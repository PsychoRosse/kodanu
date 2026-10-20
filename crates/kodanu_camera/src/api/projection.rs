use crate::PerspectiveProjection;

use kodanu_math::Mat4;

#[derive(Debug, Clone, Copy)]
pub enum Projection {
    Perspective(PerspectiveProjection),
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

impl Projection {
    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        match self {
            Projection::Perspective(projection) => projection.projection_matrix(),
        }
    }
}
