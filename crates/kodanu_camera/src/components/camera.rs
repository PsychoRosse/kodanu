use crate::Projection;

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, SurfaceSize},
};

/// A camera component used to project a scene for rendering.
///
/// `Camera` stores the projection configuration while the view transform is
/// supplied separately when constructing the view-projection matrix.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Camera {
    projection: Projection,
}

impl Camera {
    /// Combines the camera projection with a view matrix.
    ///
    /// The resulting matrix transforms coordinates from world space into
    /// clip space.
    #[must_use]
    pub fn view_projection_matrix(&self, view_matrix: Mat4) -> Mat4 {
        self.projection.projection_matrix() * view_matrix
    }

    /// Updates the camera aspect ratio from a rendering surface size.
    ///
    /// The aspect ratio is calculated as `width / height`.
    ///
    /// Does nothing when the surface height is zero.
    pub fn set_aspect_from_surface(&mut self, size: SurfaceSize) {
        let aspect = size.aspect_ratio();

        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect),
        }
    }

    /// Builds the camera's projection matrix.
    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.projection_matrix()
    }

    /// Returns the camera's projection configuration.
    #[must_use]
    pub const fn projection(&self) -> Projection {
        self.projection
    }
}

impl From<Projection> for Camera {
    /// Creates a camera using the specified projection.
    fn from(value: Projection) -> Self {
        Self { projection: value }
    }
}
