use crate::{ActivePlayer, PlayerSettings, Velocity};

use {
    kodanu_camera::CameraState,
    kodanu_ecs::ResMut,
    kodanu_math::Quat,
    kodanu_physics::{Collider, RigidBody},
};

pub use {
    kodanu_camera::{ActiveCamera, Camera, CameraSettings},
    kodanu_ecs::{Commands, Plugin, Query, Read, Registry, Res, Stage, With, Write},
    kodanu_input::{Axis, Input},
    kodanu_math::Vec3,
    kodanu_physics::PhysicsWorld,
    kodanu_time::Time,
    kodanu_transform::{Space, Transform},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(CameraState::default())
            .with_res(CameraSettings::default())
            .with_res(PlayerSettings::default());

        registry
            .with_system(Stage::Startup, player_startup_system)
            .with_system(Stage::LateUpdate, first_person_camera_system)
            .with_system(Stage::Update, player_ground_check_system)
            .with_system(Stage::Update, player_move_system);
    }
}

fn player_move_system(
    time: Res<Time>,
    input: Res<Input>,
    settings: Res<PlayerSettings>,
    query: Query<Write<Transform>, With<ActivePlayer>>,
) {
    for transform in query {
        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z);
        let left = Vec3::new(transform.left().x, 0.0, transform.left().z);

        let direction = (forward * input.axis(Axis::MoveY) + left * input.axis(Axis::MoveX))
            .try_normalize()
            .unwrap_or(Vec3::ZERO);

        transform.translate(direction * settings.speed * time.delta(), Space::Global);
    }
}

#[allow(clippy::type_complexity)]
fn player_ground_check_system(
    time: Res<Time>,
    physics: Res<PhysicsWorld>,
    query: Query<
        (
            Write<Transform>,
            Write<Velocity>,
            Read<RigidBody>,
            Read<Collider>,
        ),
        With<ActivePlayer>,
    >,
) {
    for (transform, velocity, rigid_body, collider) in query {
        if physics
            .spherecast(
                transform.position,
                1.0,
                Vec3::DOWN,
                1.08,
                Some(&[collider]),
                Some(&[rigid_body]),
            )
            .is_none()
        {
            let delta = time.delta();

            velocity.linear.y -= 9.81 * delta;
            transform.position.y += velocity.linear.y * delta;
        } else {
            velocity.linear.y = -1.0;
        }
    }
}

fn first_person_camera_system(
    time: Res<Time>,
    input: Res<Input>,
    settings: Res<CameraSettings>,
    mut state: ResMut<CameraState>,
    query: Query<Write<Transform>, With<ActiveCamera>>,
) {
    let delta = time.delta();

    state.yaw += input.axis(Axis::LookX) * settings.sens * delta;
    state.pitch += input.axis(Axis::LookY) * settings.sens * delta;

    state.pitch = state
        .pitch
        .clamp(CameraState::MIN_PITCH, CameraState::MAX_PITCH);

    for transform in query {
        transform.rotation = Quat::from_rotation_y(state.yaw) * Quat::from_rotation_x(state.pitch);
    }
}

fn player_startup_system(commands: Commands) {
    commands.spawn((
        Transform::from_position(Vec3::new(0.0, 10.0, 0.0)),
        Camera::default(),
        RigidBody::kinematic(),
        Collider::sphere(3.0),
        Velocity::default(),
        ActivePlayer,
        ActiveCamera,
    ));
}
