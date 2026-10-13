pub use {
    kodanu_ecs::{Commands, Plugin, Registry, Stage},
    kodanu_graphics::{Color, DirectLight, MeshRenderer},
    kodanu_math::{Quat, Vec3},
    kodanu_physics::{Collider, RigidBody},
    kodanu_transform::Transform,
};

/// Spawns a demo scene during startup.
///
/// The plugin registers a startup system that spawns a static platform,
/// three dynamic cubes, and a directional light. Every spawned mesh is
/// simulated with [`RigidBody`] and [`Collider`].
pub struct DemoScenePlugin;

impl Plugin for DemoScenePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::Startup, demo_scene_system);
    }
}

/// Spawns the platform, the dynamic cubes, and the directional light.
fn demo_scene_system(commands: Commands) {
    commands.spawn((
        Transform::new(
            Vec3::new(0.0, -5.0, -15.0),
            Quat::from_rotation_y(45.0),
            Vec3::new(10.0, 0.25, 10.0),
        ),
        MeshRenderer::cube(Color::GREEN),
        RigidBody::fixed(),
        Collider::cube(Vec3::new(10.0, 0.25, 10.0)),
    ));

    commands.spawn((
        Transform::from_position(Vec3::new(0.0, 1.0, -15.0)),
        MeshRenderer::cube(Color::BLUE),
        RigidBody::dynamic(),
        Collider::cube(Vec3::new(1.0, 1.0, 1.0)),
    ));

    commands.spawn((
        Transform::from_position(Vec3::new(0.75, 3.0, -15.0)),
        MeshRenderer::cube(Color::WHITE),
        RigidBody::dynamic(),
        Collider::cube(Vec3::new(1.0, 1.0, 1.0)),
    ));

    commands.spawn((
        Transform::from_position(Vec3::new(-0.75, 2.5, -15.0)),
        MeshRenderer::cube(Color::RED),
        RigidBody::dynamic(),
        Collider::cube(Vec3::new(1.0, 1.0, 1.0)),
    ));

    commands.spawn((
        Transform::default(),
        DirectLight::new(-Vec3::new(0.25, 1.0, 0.0), Color::WHITE, 1.0),
    ));
}
