use crate::MouseDelta;

use kodanu_ecs::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Event, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MousePos {
    x: f32,
    y: f32,
}

impl MousePos {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MousePos {
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }
}

impl std::ops::Sub for MousePos {
    type Output = MouseDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        MouseDelta::new(self.x - rhs.x, self.y - rhs.y)
    }
}
