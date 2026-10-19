use kodanu_ecs::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerSettings {
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self::new(10.0)
    }
}

impl PlayerSettings {
    #[must_use]
    pub const fn new(speed: f32) -> Self {
        Self { speed }
    }
}
