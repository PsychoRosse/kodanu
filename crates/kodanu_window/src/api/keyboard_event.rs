use crate::{Key, KeyStatus};

use {kodanu_ecs::Event, kodanu_macros::Getters};

#[derive(Getters, Event, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyboardEvent {
    #[getter(copy, must_use)]
    key: Key,
    #[getter(copy, must_use)]
    state: KeyStatus,
}

impl KeyboardEvent {
    #[inline]
    pub fn new(key: Key, state: KeyStatus) -> Self {
        Self { key, state }
    }
}
