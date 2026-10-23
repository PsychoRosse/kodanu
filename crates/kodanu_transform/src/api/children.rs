use kodanu_ecs::{Component, Entity};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Children(pub Vec<Entity>);
