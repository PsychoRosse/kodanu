use crate::MouseScrollDelta;

use serde::{Deserialize, Serialize};

/// Describes the unit used by a mouse scroll event.
///
/// Mouse scroll input can be reported either as discrete lines
/// or as a continuous pixel offset.
///
/// # Examples
///
/// ```
/// use kodanu_math::{MouseScroll, MouseScrollDelta};
///
/// let line_scroll = MouseScroll::Lines(MouseScrollDelta::new(0.0, -1.0));
/// let pixel_scroll = MouseScroll::Pixels(MouseScrollDelta::new(0.0, -8.0));
///
/// assert!(matches!(line_scroll, MouseScroll::Lines(_)));
/// assert!(matches!(pixel_scroll, MouseScroll::Pixels(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MouseScroll {
    /// Scroll amount reported in logical lines.
    ///
    /// The value is represented by [`crate::MouseScrollDelta`] and may be
    /// converted to pixels using [`crate::MouseScrollDelta::DEFAULT_MULTIPLE`].
    Lines(MouseScrollDelta),
    /// Scroll amount reported in physical pixels.
    ///
    /// Pixel-based scrolling is typically produced by high-resolution
    /// scrolling devices such as touchpads.
    Pixels(MouseScrollDelta),
}
