use crate::Color;

use {
    bytemuck::{Pod, Zeroable},
    kodanu_math::Vec3,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct LightUniform {
    direction: [f32; 3],
    intensity: f32,
    color: [f32; 4],
}

impl Default for LightUniform {
    #[inline]
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Color::WHITE)
    }
}

impl LightUniform {
    #[inline]
    pub fn new(direction: Vec3, intensity: f32, color: Color) -> Self {
        Self {
            direction: direction.to_array(),
            intensity,
            color: color.get(),
        }
    }
}
