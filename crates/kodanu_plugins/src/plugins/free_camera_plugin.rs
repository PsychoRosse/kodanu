pub use {
    kodanu_camera::{ActiveCamera, Camera, CameraSettings},
    kodanu_ecs::{Commands, Plugin, Query, Registry, Res, Stage, With, Write},
    kodanu_input::{Axis, Input},
    kodanu_math::Vec3,
    kodanu_physics::{Collider, RigidBody},
    kodanu_time::Time,
    kodanu_transform::{Space, Transform},
};

/// Provides a free-flying debug camera.
///
/// The plugin registers [`CameraSettings`] as a resource and
/// spawns a camera entity marked with [`ActiveCamera`] during startup. In
/// [`Stage::LateUpdate`] the camera moves along its basis directions using
/// [`Input`] axes, scaled by the settings and the frame delta time.
pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(CameraSettings::default())
            .with_system(Stage::Startup, free_camera_startup_system)
            .with_system(Stage::LateUpdate, free_camera_system);
    }
}

/// Moves and rotates the active camera using the input axes.
fn free_camera_system(
    query: Query<Write<Transform>, With<ActiveCamera>>,
    settings: Res<CameraSettings>,
    input: Res<Input>,
    time: Res<Time>,
) {
    for transform in query {
        let direction = transform.forward() * input.axis(Axis::MoveY)
            + transform.left() * input.axis(Axis::MoveX)
            + transform.up() * input.axis(Axis::MoveZ);

        let yaw = input.axis(Axis::LookX) * settings.sens * time.delta();
        let pitch = input.axis(Axis::LookY) * settings.sens * time.delta();

        transform.translate(direction * settings.speed * time.delta(), Space::Global);

        transform.rotate(Vec3::UP, yaw, Space::Global);
        transform.rotate(Vec3::RIGHT, pitch, Space::Local);
    }
}

/// Spawns the active camera with a kinematic body and a sphere collider.
fn free_camera_startup_system(commands: Commands) {
    commands.spawn((
        Transform::default(),
        Camera::default(),
        Collider::sphere(1.0),
        RigidBody::kinematic(),
        ActiveCamera,
    ));
}
