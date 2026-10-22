use crate::Color;

use kodanu_macros::Getters;

#[derive(Getters, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Material {
    #[getter(copy)]
    color: Color,
}

impl Material {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl From<Color> for Material {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}
