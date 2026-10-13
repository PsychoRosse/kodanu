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

#[cfg(test)]
mod test {
    use super::*;

    use kodanu_ecs::World;

    #[test]
    fn default_uses_documented_values() {
        let settings = CameraSettings::default();

        assert_eq!(settings.sens, 100.0);
        assert_eq!(settings.speed, 15.0);
    }

    #[test]
    fn new_stores_values() {
        let settings = CameraSettings::new(50.0, 7.5);

        assert_eq!(settings.sens, 50.0);
        assert_eq!(settings.speed, 7.5);
    }

    #[test]
    fn settings_can_be_stored_as_resource() {
        let mut world = World::default();

        world.with_res(CameraSettings::new(50.0, 7.5));

        let settings = world.expect_res::<CameraSettings>();

        assert_eq!(settings.sens, 50.0);
        assert_eq!(settings.speed, 7.5);
    }
}
