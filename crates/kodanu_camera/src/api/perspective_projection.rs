use kodanu_math::Mat4;

/// A perspective projection configuration.
///
/// The projection is defined by a vertical field of view, aspect ratio,
/// and distances to the near and far clipping planes.
///
/// The field of view is specified in degrees.
///
/// The projection uses a right-handed coordinate system.
#[derive(Debug, Clone, Copy)]
pub struct PerspectiveProjection {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(90.0, 1.0, 0.03, 1000.0)
    }
}

impl PerspectiveProjection {
    /// Creates a perspective projection.
    ///
    /// `fov` is the vertical field of view in degrees.
    /// `aspect` is the width-to-height ratio of the viewport.
    /// `near` and `far` define the distances to the clipping planes.
    ///
    /// No validation is performed on the supplied values.
    #[must_use]
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
        }
    }
}

impl PerspectiveProjection {
    /// Sets the viewport aspect ratio.
    ///
    /// The aspect ratio is calculated as `width / height`.
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// Builds the perspective projection matrix.
    ///
    /// The matrix is constructed using a right-handed perspective projection.
    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Returns the vertical field of view in degrees.
    #[must_use]
    pub const fn fov(&self) -> f32 {
        self.fov
    }

    /// Returns the viewport aspect ratio.
    #[must_use]
    pub const fn aspect(&self) -> f32 {
        self.aspect
    }

    /// Returns the distance to the near clipping plane.
    #[must_use]
    pub const fn near(&self) -> f32 {
        self.near
    }

    /// Returns the distance to the far clipping plane.
    #[must_use]
    pub const fn far(&self) -> f32 {
        self.far
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_uses_documented_values() {
        let projection = PerspectiveProjection::default();

        assert_eq!(projection.fov(), 90.0);
        assert_eq!(projection.aspect(), 1.0);
        assert_eq!(projection.near(), 0.03);
        assert_eq!(projection.far(), 1000.0);
    }

    #[test]
    fn getters_return_stored_components() {
        let projection = PerspectiveProjection::new(60.0, 1.5, 0.1, 500.0);

        assert_eq!(projection.fov(), 60.0);
        assert_eq!(projection.aspect(), 1.5);
        assert_eq!(projection.near(), 0.1);
        assert_eq!(projection.far(), 500.0);
    }

    #[test]
    fn set_aspect_ratio_updates_aspect() {
        let mut projection = PerspectiveProjection::default();

        projection.set_aspect_ratio(16.0 / 9.0);

        assert_eq!(projection.aspect(), 16.0 / 9.0);
    }

    #[test]
    fn projection_matrix_matches_perspective_rh() {
        let projection = PerspectiveProjection::new(60.0, 2.0, 0.1, 500.0);

        assert_eq!(
            projection.projection_matrix(),
            Mat4::perspective_rh(60.0, 2.0, 0.1, 500.0)
        );
    }

    #[test]
    fn aspect_change_alters_projection_matrix() {
        let mut projection = PerspectiveProjection::default();
        let before = projection.projection_matrix();

        projection.set_aspect_ratio(2.0);

        assert_ne!(before, projection.projection_matrix());
    }
}
