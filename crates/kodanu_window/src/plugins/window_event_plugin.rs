use crate::{Closing, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing};

pub use {
    kodanu_ecs::{EventBuffer, Plugin, Registry},
    kodanu_math::{MousePos, SurfaceSize},
};

#[derive(Clone, Copy)]
pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_event(EventBuffer::<Closing>::default())
            .with_event(EventBuffer::<Redrawing>::default())
            .with_event(EventBuffer::<SurfaceSize>::default());

        registry
            .with_event(EventBuffer::<KeyboardEvent>::default())
            .with_event(EventBuffer::<MouseEvent>::default())
            .with_event(EventBuffer::<MousePos>::default())
            .with_event(EventBuffer::<MouseWheelEvent>::default());
    }
}
