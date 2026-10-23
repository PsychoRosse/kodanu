use kodanu_ecs::Resource;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraSettings {
    pub sens: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self::new(200.0)
    }
}

impl CameraSettings {
    #[must_use]
    pub const fn new(sens: f32) -> Self {
        Self { sens }
    }
}
