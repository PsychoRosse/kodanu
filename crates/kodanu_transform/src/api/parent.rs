use kodanu_ecs::{Component, Entity};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parent(pub Entity);
