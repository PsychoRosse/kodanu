use {
    kodanu_ecs::Event,
    serde::{Deserialize, Serialize},
};

/// An event emitted when the rendering surface changes size.
///
/// The dimensions are specified in physical pixels.
///
/// Both dimensions are guaranteed to be at least `1`.
///
/// # Examples
///
/// ```
/// use kodanu_math::SurfaceSize;
///
/// let event = SurfaceSize::clamped(1280, 720);
///
/// assert_eq!(event.width(), 1280);
/// assert_eq!(event.height(), 720);
/// ```
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SurfaceSize {
    width: u32,
    height: u32,
}

impl SurfaceSize {
    /// Creates a surface size from the given dimensions.
    ///
    /// Each dimension is clamped to a minimum of `1` pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use kodanu_math::SurfaceSize;
    ///
    /// let size = SurfaceSize::clamped(1920, 1080);
    ///
    /// assert_eq!(size.width(), 1920);
    /// assert_eq!(size.height(), 1080);
    /// ```
    ///
    /// Zero dimensions are clamped to `1`:
    ///
    /// ```
    /// use kodanu_math::SurfaceSize;
    ///
    /// let size = SurfaceSize::clamped(0, 0);
    ///
    /// assert_eq!(size.width(), 1);
    /// assert_eq!(size.height(), 1);
    /// ```
    #[must_use]
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl Default for SurfaceSize {
    /// Returns the default surface size.
    ///
    /// The default dimensions are `640 × 520` pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use kodanu_math::SurfaceSize;
    ///
    /// let size = SurfaceSize::default();
    ///
    /// assert_eq!(size.width(), 640);
    /// assert_eq!(size.height(), 520);
    /// ```
    fn default() -> Self {
        Self::clamped(640, 520)
    }
}

impl SurfaceSize {
    /// Returns the surface width in physical pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the surface height in physical pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the width-to-height aspect ratio.
    ///
    /// The result is always greater than zero.
    #[must_use]
    pub const fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
