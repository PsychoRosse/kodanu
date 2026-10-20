use kodanu::ecs::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerSettings {
    pub speed: f32,
    pub jump_height: f32,
    pub smooth_factor: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self::new(10.0, 2.0, 10.0)
    }
}

impl PlayerSettings {
    #[must_use]
    pub const fn new(speed: f32, jump_height: f32, smooth_factor: f32) -> Self {
        Self {
            speed,
            jump_height,
            smooth_factor,
        }
    }
}
