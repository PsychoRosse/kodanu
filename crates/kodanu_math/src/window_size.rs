use {
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WindowSize {
    width: u32,
    height: u32,
}

impl WindowSize {
    pub const HD: Self = Self {
        width: 1280,
        height: 720,
    };
    pub const FHD: Self = Self {
        width: 1920,
        height: 1080,
    };
    pub const QHD: Self = Self {
        width: 2560,
        height: 1440,
    };
    pub const UHD: Self = Self {
        width: 3840,
        height: 2160,
    };
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 640,
            height: 520,
        }
    }
}

impl WindowSize {
    #[must_use]
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }

    #[must_use]
    pub fn with_width(self, width: u32) -> Self {
        Self {
            width: width.max(1),
            height: self.height,
        }
    }

    #[must_use]
    pub fn with_height(self, height: u32) -> Self {
        Self {
            width: self.width,
            height: height.max(1),
        }
    }
}

impl WindowSize {
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    #[must_use]
    pub const fn is_landscape(self) -> bool {
        self.width > self.height
    }

    #[must_use]
    pub const fn is_portrait(self) -> bool {
        self.height > self.width
    }

    #[must_use]
    pub const fn is_square(self) -> bool {
        self.width == self.height
    }

    #[must_use]
    pub fn min_dimension(self) -> u32 {
        self.width.min(self.height)
    }

    #[must_use]
    pub fn max_dimension(self) -> u32 {
        self.width.max(self.height)
    }
}

impl From<[u32; 2]> for WindowSize {
    fn from([width, height]: [u32; 2]) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for [u32; 2] {
    fn from(size: WindowSize) -> Self {
        [size.width, size.height]
    }
}

impl From<(u32, u32)> for WindowSize {
    fn from((width, height): (u32, u32)) -> Self {
        Self::clamped(width, height)
    }
}

impl From<WindowSize> for (u32, u32) {
    fn from(size: WindowSize) -> Self {
        (size.width, size.height)
    }
}

impl Display for WindowSize {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}
