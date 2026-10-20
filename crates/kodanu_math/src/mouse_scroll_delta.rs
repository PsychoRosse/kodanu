use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MouseScrollDelta {
    x: f32,
    y: f32,
}

impl MouseScrollDelta {
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseScrollDelta {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MouseScrollDelta {
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }
}
