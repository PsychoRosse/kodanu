use {kodanu_ecs::Component, kodanu_math::Vec3};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PlayerState {
    direction: Vec3,
    is_ground: bool,
}

impl PlayerState {
    #[inline]
    #[must_use]
    pub const fn is_ground(&self) -> bool {
        self.is_ground
    }

    #[inline]
    pub const fn set(&mut self, is_ground: bool) {
        self.is_ground = is_ground
    }
}
