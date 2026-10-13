pub use kodanu_ecs::Component;

/// Marks an entity as the active camera.
///
/// The component itself does not store any camera state. Systems can use its
/// presence to identify the camera used for rendering.
#[derive(Component, Default, Clone, Copy)]
pub struct ActiveCamera;

#[cfg(test)]
mod test {
    use super::*;

    use {crate::Camera, kodanu_ecs::World};

    #[test]
    fn marker_can_be_spawned_and_read_back() {
        let mut world = World::default();

        let entity = world.spawn((ActiveCamera,));

        assert!(world.component::<ActiveCamera>(entity).is_some());
    }

    #[test]
    fn marker_is_absent_on_entity_without_it() {
        let mut world = World::default();

        let entity = world.spawn((Camera::default(),));

        assert!(world.component::<ActiveCamera>(entity).is_none());
    }
}
