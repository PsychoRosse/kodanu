use {
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

/// The size of a window in physical pixels.
///
/// Both dimensions are guaranteed to be at least `1`.
///
/// # Coordinate Order
///
/// Dimensions are represented as `(width, height)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WindowSize {
    width: u32,
    height: u32,
}

impl WindowSize {
    /// High Definition (HD), `1280 × 720`.
    pub const HD: Self = Self {
        width: 1280,
        height: 720,
    };
    /// Full High Definition (FHD), `1920 × 1080`.
    pub const FHD: Self = Self {
        width: 1920,
        height: 1080,
    };
    /// Quad High Definition (QHD), `2560 × 1440`.
    pub const QHD: Self = Self {
        width: 2560,
        height: 1440,
    };
    /// Ultra High Definition (UHD), `3840 × 2160`.
    pub const UHD: Self = Self {
        width: 3840,
        height: 2160,
    };
}

impl Default for WindowSize {
    /// Returns the default window size.
    ///
    /// The default dimensions are `640 × 520` pixels.
    fn default() -> Self {
        Self {
            width: 640,
            height: 520,
        }
    }
}

impl WindowSize {
    /// Creates a window size from the given dimensions.
    ///
    /// Each dimension is clamped to a minimum of `1` pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use kodanu_math::WindowSize;
    ///
    /// let size = WindowSize::clamped(1280, 720);
    ///
    /// assert_eq!(size.width(), 1280);
    /// assert_eq!(size.height(), 720);
    /// ```
    ///
    /// Values smaller than `1` are automatically clamped:
    ///
    /// ```
    /// use kodanu_math::WindowSize;
    ///
    /// let size = WindowSize::clamped(0, 0);
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

    /// Returns a copy of this size with a different width.
    ///
    /// The new width is clamped to a minimum of `1`.
    #[must_use]
    pub fn with_width(self, width: u32) -> Self {
        Self {
            width: width.max(1),
            height: self.height,
        }
    }

    /// Returns a copy of this size with a different height.
    ///
    /// The new height is clamped to a minimum of `1`.
    #[must_use]
    pub fn with_height(self, height: u32) -> Self {
        Self {
            width: self.width,
            height: height.max(1),
        }
    }
}

impl WindowSize {
    /// Returns the window width in physical pixels.
    ///
    /// The returned value is guaranteed to be at least `1`.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the window height in physical pixels.
    ///
    /// The returned value is guaranteed to be at least `1`.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the number of physical pixels represented by this size.
    ///
    /// The result is returned as `u64` to avoid overflowing a `u32`
    /// multiplication.
    #[must_use]
    pub const fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Returns `true` if the width is greater than the height.
    #[must_use]
    pub const fn is_landscape(self) -> bool {
        self.width > self.height
    }

    /// Returns `true` if the height is greater than the width.
    #[must_use]
    pub const fn is_portrait(self) -> bool {
        self.height > self.width
    }

    /// Returns `true` if the width and height are equal.
    #[must_use]
    pub const fn is_square(self) -> bool {
        self.width == self.height
    }

    /// Returns the smaller of the two dimensions.
    #[must_use]
    pub fn min_dimension(self) -> u32 {
        self.width.min(self.height)
    }

    /// Returns the larger of the two dimensions.
    #[must_use]
    pub fn max_dimension(self) -> u32 {
        self.width.max(self.height)
    }
}

impl From<[u32; 2]> for WindowSize {
    /// Creates a window size from `[width, height]`.
    ///
    /// Both dimensions are clamped to a minimum of `1`.
    fn from([width, height]: [u32; 2]) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for [u32; 2] {
    /// Converts the window size into `[width, height]`.
    fn from(size: WindowSize) -> Self {
        [size.width, size.height]
    }
}

impl From<(u32, u32)> for WindowSize {
    /// Creates a window size from `(width, height)`.
    ///
    /// Both dimensions are clamped to a minimum of `1`.
    fn from((width, height): (u32, u32)) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for (u32, u32) {
    /// Converts the window size into `(width, height)`.
    fn from(size: WindowSize) -> Self {
        (size.width, size.height)
    }
}

impl Display for WindowSize {
    /// Formats the size as `width×height`, for example `1920×1080`.
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}
