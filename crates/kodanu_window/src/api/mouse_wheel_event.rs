use {kodanu_ecs::Event, kodanu_macros::Getters, kodanu_math::MouseScroll};

#[derive(Getters, Event, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseWheelEvent {
    #[getter(copy, must_use)]
    scroll: MouseScroll,
}

impl From<MouseScroll> for MouseWheelEvent {
    #[inline]
    fn from(value: MouseScroll) -> Self {
        Self { scroll: value }
    }
}
