use crate::{Button, ButtonStatus};

use {kodanu_ecs::Event, kodanu_macros::Getters};

#[derive(Getters, Event, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MouseEvent {
    #[getter(copy, must_use)]
    button: Button,
    #[getter(copy, must_use)]
    state: ButtonStatus,
}

impl MouseEvent {
    #[inline]
    pub fn new(button: Button, state: ButtonStatus) -> Self {
        Self { button, state }
    }
}
