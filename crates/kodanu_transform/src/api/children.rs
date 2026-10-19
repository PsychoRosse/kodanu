use kodanu_ecs::{Component, Entity};

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Children(pub Vec<Entity>);
