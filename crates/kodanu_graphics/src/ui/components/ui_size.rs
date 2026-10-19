#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiSize {
    Fixed(f32),
    Fill,
}

impl Default for UiSize {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}
