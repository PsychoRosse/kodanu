use {
    kodanu_macros::Getters,
    raw_window_handle::{RawDisplayHandle, RawWindowHandle},
};

#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceTarget {
    #[getter(copy, must_use)]
    raw_display: RawDisplayHandle,
    #[getter(copy, must_use)]
    raw_window: RawWindowHandle,
}

impl SurfaceTarget {
    pub fn new(raw_display: RawDisplayHandle, raw_window: RawWindowHandle) -> Self {
        Self {
            raw_display,
            raw_window,
        }
    }
}
