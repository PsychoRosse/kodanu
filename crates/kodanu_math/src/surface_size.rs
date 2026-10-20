use {
    kodanu_ecs::Event,
    serde::{Deserialize, Serialize},
};

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SurfaceSize {
    width: u32,
    height: u32,
}

impl SurfaceSize {
    #[must_use]
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl Default for SurfaceSize {
    fn default() -> Self {
        Self::clamped(640, 520)
    }
}

impl SurfaceSize {
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
