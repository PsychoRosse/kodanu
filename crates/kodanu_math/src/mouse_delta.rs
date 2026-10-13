use crate::MousePos;

use {
    serde::{Deserialize, Serialize},
    std::ops::AddAssign,
};

/// Represents a mouse movement delta in window coordinates.
///
/// The delta describes a change in cursor position, measured in physical
/// pixels. Its components are private and can be accessed through
/// [`Self::x`] and [`Self::y`].
///
/// Unlike [`MousePos`], which represents an absolute cursor position,
/// `MouseDelta` represents a relative offset.
///
/// # Examples
///
/// ```
/// use kodanu_math::MouseDelta;
///
/// let mut delta = MouseDelta::new(4.0, -2.0);
/// delta += MouseDelta::new(1.0, 3.0);
///
/// assert_eq!(delta.x(), 5.0);
/// assert_eq!(delta.y(), 1.0);
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MouseDelta {
    x: f32,
    y: f32,
}

impl MouseDelta {
    /// Creates a mouse movement delta from its horizontal and vertical
    /// components, measured in physical pixels.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a mouse movement delta from its horizontal and vertical
    /// components, or returns `None` if either component is not finite.
    #[must_use]
    pub fn try_new(x: f32, y: f32) -> Option<Self> {
        (x.is_finite() && y.is_finite()).then_some(Self::new(x, y))
    }
}

impl MouseDelta {
    /// Returns the horizontal movement delta in physical pixels.
    #[must_use]
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the vertical movement delta in physical pixels.
    #[must_use]
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Creates a mouse movement delta from its horizontal and vertical
    /// components.
    ///
    /// This is equivalent to [`Self::new`].
    #[must_use]
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }

    /// Returns `true` if every component is finite.
    ///
    /// A component is finite when it is neither `NaN` nor infinite.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl AddAssign for MouseDelta {
    /// Adds another movement delta component-wise.
    ///
    /// This is useful for accumulating mouse movement across multiple
    /// input events.
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<MousePos> for MouseDelta {
    /// Creates a delta from the absolute cursor coordinates.
    ///
    /// This conversion copies the position's coordinates without
    /// calculating movement relative to a previous position.
    fn from(pos: MousePos) -> Self {
        Self::new(pos.x(), pos.y())
    }
}
