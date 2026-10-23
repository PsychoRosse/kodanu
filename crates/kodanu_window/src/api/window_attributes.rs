use crate::winit;

use {kodanu_macros::Getters, kodanu_math::WindowSize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowAttributes {
    #[getter(str)]
    title: &'static str,
    #[getter(copy)]
    size: WindowSize,
    #[getter(copy)]
    min_size: WindowSize,
    #[getter(copy)]
    maximized: bool,
    #[getter(copy)]
    decorations: bool,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: "Engine",
            size: WindowSize::default(),
            min_size: WindowSize::default(),
            maximized: false,
            decorations: true,
        }
    }
}

impl WindowAttributes {
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize::clamped(width, height);
        self
    }

    pub fn with_min_size(mut self, min_width: u32, min_height: u32) -> Self {
        self.min_size = WindowSize::clamped(min_width, min_height);
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }
}

impl From<WindowAttributes> for winit::WindowAttributes {
    fn from(value: WindowAttributes) -> Self {
        let surface_size = winit::PhysicalSize::new(value.size.width(), value.size.height());
        
        let min_surface_size =
            winit::PhysicalSize::new(value.min_size.width(), value.min_size.height());

        winit::WindowAttributes::default()
            .with_title(value.title)
            .with_surface_size(surface_size)
            .with_min_surface_size(min_surface_size)
            .with_maximized(value.maximized)
            .with_decorations(value.decorations)
    }
}
