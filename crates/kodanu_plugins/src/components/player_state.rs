use {kodanu_ecs::Resource, kodanu_math::Vec3};

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PlayerState {
    direction: Vec3,
    is_ground: bool,
}

impl PlayerState {
    pub fn is_ground(&self) -> bool {
        self.is_ground
    }

    pub fn set(&mut self, is_ground: bool) {
        self.is_ground = is_ground
    }
}
