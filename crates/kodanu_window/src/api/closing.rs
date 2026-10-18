use kodanu_ecs::Event;

#[derive(Event, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Closing;
