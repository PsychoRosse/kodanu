use {
    kodanu_macros::Getters,
    serde::{Deserialize, Serialize},
};

#[derive(Getters, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Color {
    #[getter(copy, name = "get")]
    value: [f32; 4],
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::rgba(0.0, 0.0, 1.0, 1.0);
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            value: [r, g, b, 1.0],
        }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            value: [r, g, b, a],
        }
    }
}
