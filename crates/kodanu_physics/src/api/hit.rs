use kodanu_math::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

impl Hit {
    #[inline]
    pub fn new(point: Vec3, normal: Vec3, distance: f32) -> Self {
        Self {
            point,
            normal,
            distance,
        }
    }
}
