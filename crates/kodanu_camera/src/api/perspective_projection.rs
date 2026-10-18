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
