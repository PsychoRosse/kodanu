use crate::{Collider, Hit, RigidBody};

pub use {
    kodanu_ecs::{Query, Read, Res, ResMut, Resource, Write},
    kodanu_math::{Quat, Vec3},
    kodanu_transform::Transform,
};

use rapier3d::{
    math::{Rot3 as RapierRot3, Vec3 as RapierVec3, Vector as RapierVector},
    parry::query::Ray as RapierRay,
    pipeline::QueryFilter as RapierQueryFilter,
};

use rapier3d::prelude::{
    PhysicsWorld as RapierPhysicsWorld, Pose3 as RapierPose3, RigidBodyType as RapierRigidBodyType,
};

#[derive(Resource, Default)]
pub struct PhysicsWorld {
    physics: RapierPhysicsWorld,
}

impl PhysicsWorld {
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.physics.gravity = RapierVec3::new(gravity.x, gravity.y, gravity.z);
    }

    pub fn remove_rigid_body(&mut self, body: &mut RigidBody) {
        if let Some(handle) = body.handle() {
            self.physics.remove_body(handle);
            body.set_handle(None);
        }
    }

    pub fn remove_collider(&mut self, collider: &mut Collider) {
        if let Some(handle) = collider.handle() {
            self.physics.remove_collider(handle);
            collider.set_handle(None);
        }
    }

    pub fn raycast(&self, origin: Vec3, direction: Vec3, max_distance: f32) -> Option<Hit> {
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }

        if !origin.is_finite() || !direction.is_finite() {
            return None;
        }

        let length_squared = direction.length_squared();

        if length_squared <= f32::EPSILON {
            return None;
        }

        let inv_length = length_squared.sqrt().recip();
        let origin = RapierVector::new(origin.x, origin.y, origin.z);

        let direction = RapierVector::new(
            direction.x * inv_length,
            direction.y * inv_length,
            direction.z * inv_length,
        );

        let ray = RapierRay::new(origin, direction);

        let query = self.physics.broad_phase.as_query_pipeline(
            self.physics.narrow_phase.query_dispatcher(),
            &self.physics.bodies,
            &self.physics.colliders,
            RapierQueryFilter::default(),
        );

        let (_, intersection) = query.cast_ray_and_get_normal(&ray, max_distance, true)?;

        let point = ray.point_at(intersection.time_of_impact);
        let normal = intersection.normal;

        Some(Hit::new(
            Vec3::new(point.x, point.y, point.z),
            Vec3::new(normal.x, normal.y, normal.z),
            intersection.time_of_impact,
        ))
    }
}

impl PhysicsWorld {
    pub(crate) fn collider_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<RigidBody>, Write<Collider>)>,
    ) {
        for (rigidbody, collider) in query {
            if collider.handle().is_some() {
                continue;
            }

            collider.set_handle(Some(
                world
                    .physics
                    .insert_collider(collider.builder().build(), rigidbody.handle()),
            ));
        }
    }

    pub(crate) fn rigidbody_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            if rigid_body.handle().is_some() {
                continue;
            }

            rigid_body.set_handle(Some(
                world
                    .physics
                    .insert_body(rigid_body.builder(transform).build()),
            ));
        }
    }

    pub(crate) fn dynamic_physics_system(
        world: Res<PhysicsWorld>,
        query: Query<(Write<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            let Some(handle) = rigid_body.handle() else {
                continue;
            };

            if rigid_body.body_type() != RapierRigidBodyType::Dynamic {
                continue;
            }

            let Some(body) = world.physics.bodies.get(handle) else {
                continue;
            };

            let position = body.position();

            transform.set_position_and_rotation(
                Vec3::new(
                    position.translation.x,
                    position.translation.y,
                    position.translation.z,
                ),
                Quat::new(
                    position.rotation.x,
                    position.rotation.y,
                    position.rotation.z,
                    position.rotation.w,
                ),
            );
        }
    }

    pub(crate) fn kinematic_physics_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            let Some(handle) = rigid_body.handle() else {
                continue;
            };

            if rigid_body.body_type() != RapierRigidBodyType::KinematicPositionBased {
                continue;
            }

            let Some(body) = world.physics.bodies.get_mut(handle) else {
                continue;
            };

            body.set_next_kinematic_position(RapierPose3::from_parts(
                RapierVec3::new(
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                ),
                RapierRot3::from_xyzw(
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z,
                    transform.rotation.w,
                ),
            ));
        }
    }

    pub(crate) fn physics_system(mut world: ResMut<PhysicsWorld>) {
        world.physics.step();
    }
}
