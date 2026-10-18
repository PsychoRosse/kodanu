use {
    kodanu_macros::Getters,
    raw_window_handle::{RawDisplayHandle, RawWindowHandle},
};

#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceTarget {
    #[getter(copy, must_use, name = "raw_display")]
    display: RawDisplayHandle,
    #[getter(copy, must_use, name = "raw_window")]
    window: RawWindowHandle,
}

impl SurfaceTarget {
    pub fn new(display: RawDisplayHandle, window: RawWindowHandle) -> Self {
        Self { display, window }
    }
}
