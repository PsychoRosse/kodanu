use crate::Color;

use {
    kodanu_macros::Getters,
    {kodanu_ecs::Component, kodanu_math::Vec3},
};

#[derive(Getters, Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DirectLight {
    #[getter(copy)]
    direction: Vec3,
    #[getter(copy)]
    color: Color,
    #[getter(copy)]
    intensity: f32,
}

impl Default for DirectLight {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            color: Color::WHITE,
            intensity: 1.0,
        }
    }
}

impl DirectLight {
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction,
            color,
            intensity,
        }
    }
}

impl DirectLight {
    pub const fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction
    }

    pub const fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub const fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity
    }
}
