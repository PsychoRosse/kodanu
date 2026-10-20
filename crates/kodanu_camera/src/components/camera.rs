use crate::Projection;

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, SurfaceSize},
};

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Camera {
    projection: Projection,
}

impl Camera {
    #[must_use]
    pub fn view_projection_matrix(&self, view_matrix: Mat4) -> Mat4 {
        self.projection.projection_matrix() * view_matrix
    }

    pub fn set_aspect_from_surface(&mut self, size: SurfaceSize) {
        let aspect = size.aspect_ratio();

        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect),
        }
    }

    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.projection_matrix()
    }

    #[must_use]
    pub const fn projection(&self) -> Projection {
        self.projection
    }
}

impl From<Projection> for Camera {
    fn from(value: Projection) -> Self {
        Self { projection: value }
    }
}
