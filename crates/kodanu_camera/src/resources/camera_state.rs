use kodanu_ecs::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
}

impl CameraState {
    pub const MIN_PITCH: f32 = -89.0;
    pub const MAX_PITCH: f32 = 89.0;
}

impl Default for CameraState {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl CameraState {
    #[must_use]
    pub const fn new(pitch: f32, yaw: f32) -> Self {
        Self { pitch, yaw }
    }
}
