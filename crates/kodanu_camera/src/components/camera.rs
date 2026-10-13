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

#[cfg(test)]
mod test {
    use super::*;

    use {crate::PerspectiveProjection, kodanu_math::Vec3};

    #[test]
    fn default_camera_uses_documented_projection() {
        let camera = Camera::default();

        assert_eq!(
            camera.projection_matrix(),
            Mat4::perspective_rh(90.0, 1.0, 0.03, 1000.0)
        );
    }

    #[test]
    fn from_projection_stores_projection() {
        let projection = Projection::Perspective(PerspectiveProjection::new(60.0, 2.0, 0.1, 500.0));
        let camera = Camera::from(projection);

        let Projection::Perspective(stored) = camera.projection();

        assert_eq!(stored.fov(), 60.0);
        assert_eq!(stored.aspect(), 2.0);
        assert_eq!(stored.near(), 0.1);
        assert_eq!(stored.far(), 500.0);
        assert_eq!(
            camera.projection_matrix(),
            Mat4::perspective_rh(60.0, 2.0, 0.1, 500.0)
        );
    }

    #[test]
    fn view_projection_matrix_combines_projection_and_view() {
        let camera = Camera::default();
        let view = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(
            camera.view_projection_matrix(Mat4::IDENTITY),
            camera.projection_matrix()
        );
        assert_eq!(
            camera.view_projection_matrix(view),
            camera.projection_matrix() * view
        );
    }

    #[test]
    fn set_aspect_from_surface_updates_aspect() {
        let mut camera = Camera::default();

        camera.set_aspect_from_surface(SurfaceSize::clamped(1920, 1080));

        let Projection::Perspective(projection) = camera.projection();

        assert_eq!(projection.aspect(), 1920.0 / 1080.0);
        assert_eq!(
            camera.projection_matrix(),
            Mat4::perspective_rh(90.0, 1920.0 / 1080.0, 0.03, 1000.0)
        );
    }
}
