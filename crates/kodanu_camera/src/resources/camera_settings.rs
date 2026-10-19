use kodanu_ecs::Resource;

/// Runtime settings for camera movement and input sensitivity.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraSettings {
    /// Mouse or input sensitivity used to control the camera.
    pub sens: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self::new(200.0)
    }
}

impl CameraSettings {
    /// Creates camera settings with the specified sensitivity speed.
    #[must_use]
    pub const fn new(sens: f32) -> Self {
        Self { sens }
    }
}
