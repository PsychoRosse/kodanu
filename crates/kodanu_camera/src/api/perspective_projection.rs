use kodanu_math::Mat4;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    #[must_use]
    pub const fn fov(&self) -> f32 {
        self.fov
    }

    #[must_use]
    pub const fn aspect(&self) -> f32 {
        self.aspect
    }

    #[must_use]
    pub const fn near(&self) -> f32 {
        self.near
    }

    #[must_use]
    pub const fn far(&self) -> f32 {
        self.far
    }
}
