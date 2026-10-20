use {kodanu_ecs::Component, kodanu_math::Vec3};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Velocity {
    pub linear: Vec3,
}

impl Velocity {
    pub const GRAVITY: f32 = 14.81;
}

impl Velocity {
    pub fn new(linear: Vec3) -> Self {
        Self { linear }
    }
}
