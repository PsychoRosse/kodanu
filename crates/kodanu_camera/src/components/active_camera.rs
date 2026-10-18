pub use kodanu_ecs::Component;

/// Marks an entity as the active camera.
///
/// The component itself does not store any camera state. Systems can use its
/// presence to identify the camera used for rendering.
#[derive(Component, Default, Clone, Copy)]
pub struct ActiveCamera;
