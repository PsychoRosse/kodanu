use crate::{ActivePlayer, PlayerSettings, PlayerState, Velocity};

use kodanu::{
    camera::{ActiveCamera, Camera, CameraSettings, CameraState},
    ecs::{Commands, Plugin, Query, Read, Registry, Res, Stage, With, Write},
    input::{Axis, Input},
    math::{Quat, Vec3},
    physics::{Collider, PhysicsWorld, RigidBody},
    time::Time,
    transform::Transform,
    window::Key,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(CameraSettings::default())
            .with_res(PlayerSettings::default());

        registry
            .with_system(Stage::Startup, player_startup_system)
            .with_system(Stage::LateUpdate, first_person_camera_system)
            .with_system(Stage::Update, player_ground_check_system)
            .with_system(Stage::Update, player_move_system);
    }
}

#[allow(clippy::type_complexity)]
fn player_move_system(
    time: Res<Time>,
    input: Res<Input>,
    settings: Res<PlayerSettings>,
    query: Query<(Write<Transform>, Write<Velocity>, Read<PlayerState>), With<ActivePlayer>>,
) {
    for (transform, velocity, state) in query {
        let delta = time.delta();

        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z);
        let left = Vec3::new(transform.left().x, 0.0, transform.left().z);

        let direction = (forward * input.axis(Axis::MoveY) + left * input.axis(Axis::MoveX))
            .try_normalize()
            .unwrap_or(Vec3::ZERO);

        let smoth_factor = (-settings.smooth_factor * delta).exp();
        let target_velocity = direction * settings.speed;

        velocity.linear.x += (target_velocity.x - velocity.linear.x) * smoth_factor;
        velocity.linear.z += (target_velocity.z - velocity.linear.z) * smoth_factor;

        if input.key_pressed(Key::Space) && state.is_ground() {
            velocity.linear.y = (2.0 * Velocity::BASE_GRAVITY * settings.jump_height).sqrt();
        }

        transform.position += velocity.linear * delta;
    }
}

#[allow(clippy::type_complexity)]
fn player_ground_check_system(
    physics: Res<PhysicsWorld>,
    time: Res<Time>,
    query: Query<
        (
            Write<Transform>,
            Write<Velocity>,
            Write<PlayerState>,
            Read<RigidBody>,
            Read<Collider>,
        ),
        With<ActivePlayer>,
    >,
) {
    for (transform, velocity, state, rigid_body, collider) in query {
        let grounded = physics
            .spherecast(
                transform.position,
                1.0,
                1.08,
                Vec3::DOWN,
                Some(&[collider]),
                Some(&[rigid_body]),
            )
            .is_some()
            && velocity.linear.y <= 0.0;

        if grounded {
            state.set(true);
            velocity.linear.y = 0.0;
        } else {
            state.set(false);
            velocity.linear.y -= Velocity::BASE_GRAVITY * time.delta();
        }
    }
}

fn first_person_camera_system(
    time: Res<Time>,
    input: Res<Input>,
    settings: Res<CameraSettings>,
    query: Query<(Write<Transform>, Write<CameraState>), With<ActiveCamera>>,
) {
    for (transform, state) in query {
        let delta = time.delta();

        state.yaw += input.axis(Axis::LookX) * settings.sens * delta;
        state.pitch += input.axis(Axis::LookY) * settings.sens * delta;

        state.pitch = state
            .pitch
            .clamp(CameraState::MIN_PITCH, CameraState::MAX_PITCH);

        transform.rotation = Quat::from_rotation_y(state.yaw) * Quat::from_rotation_x(state.pitch);
    }
}

fn player_startup_system(commands: Commands) {
    commands.spawn((
        Transform::from_position(Vec3::new(0.0, 10.0, 0.0)),
        Camera::default(),
        CameraState::default(),
        PlayerState::default(),
        RigidBody::kinematic(),
        Collider::sphere(3.0),
        Velocity::default(),
        ActivePlayer,
        ActiveCamera,
    ));
}
