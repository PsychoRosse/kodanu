use serde::{Deserialize, Serialize};

/// Represents a mouse scroll offset.
///
/// The meaning of the values depends on the [`crate::MouseScroll`] variant
/// that produced the delta.
///
/// Positive and negative values represent opposite scroll directions.
/// The exact direction depends on the input backend and engine
/// coordinate convention.
///
/// # Examples
///
/// ```
/// use kodanu_math::MouseScrollDelta;
///
/// let delta = MouseScrollDelta::new(0.0, -1.0);
///
/// assert_eq!(delta.x(), 0.0);
/// assert_eq!(delta.y(), -1.0);
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MouseScrollDelta {
    x: f32,
    y: f32,
}

impl MouseScrollDelta {
    /// Default number of pixels used to represent one scroll line.
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseScrollDelta {
    /// Creates a new scroll delta from its horizontal and vertical
    /// components.
    ///
    /// The units of the components depend on the [`crate::MouseScroll`]
    /// variant that produced the delta.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MouseScrollDelta {
    /// Returns the horizontal scroll offset.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns the vertical scroll offset.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }
}
