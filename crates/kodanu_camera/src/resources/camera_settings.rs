use kodanu_ecs::Resource;

/// Runtime settings for camera movement and input sensitivity.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraSettings {
    /// Mouse or input sensitivity used to control the camera.
    pub sens: f32,

    /// Camera movement speed.
    pub speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self::new(100.0, 15.0)
    }
}

impl CameraSettings {
    /// Creates camera settings with the specified sensitivity and movement speed.
    #[must_use]
    pub const fn new(sens: f32, speed: f32) -> Self {
        Self { sens, speed }
    }
}
