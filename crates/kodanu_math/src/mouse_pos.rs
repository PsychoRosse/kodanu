use {
    crate::MouseDelta,
    kodanu_ecs::Event,
    serde::{Deserialize, Serialize},
    std::ops::Sub,
};

/// Represents the mouse cursor position in window coordinates.
///
/// The position is expressed in physical pixels relative to the
/// top-left corner of the window.
///
/// # Examples
///
/// ```
/// use kodanu_math::MousePos;
///
/// let position = MousePos::new(640.0, 360.0);
///
/// assert_eq!(position.x(), 640.0);
/// assert_eq!(position.y(), 360.0);
/// ```
#[derive(Event, Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MousePos {
    x: f32,
    y: f32,
}

impl MousePos {
    /// Creates a mouse position from the given coordinates.
    ///
    /// Coordinates are specified in physical pixels.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MousePos {
    /// Returns the horizontal position in physical pixels.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns the vertical position in physical pixels.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }
}

impl Sub for MousePos {
    type Output = MouseDelta;

    /// Returns the component-wise difference between two positions.
    ///
    /// The resulting position is equivalent to subtracting the
    /// corresponding coordinates independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use kodanu_math::MousePos;
    ///
    /// let current = MousePos::new(100.0, 80.0);
    /// let previous = MousePos::new(60.0, 50.0);
    ///
    /// let delta = current - previous;
    ///
    /// assert_eq!(delta.x(), 40.0);
    /// assert_eq!(delta.y(), 30.0);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        MouseDelta::new(self.x - rhs.x, self.y - rhs.y)
    }
}
