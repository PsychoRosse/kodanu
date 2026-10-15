use crate::Color;

#[derive(Default, Debug, Clone, Copy)]
pub struct Material {
    color: Color,
}

impl Material {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material {
    pub const fn color(&self) -> Color {
        self.color
    }
}

impl From<Color> for Material {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}
