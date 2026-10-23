pub use kodanu_ecs::Component;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Component, Default, Clone, Copy)]
pub struct ActiveCamera;
