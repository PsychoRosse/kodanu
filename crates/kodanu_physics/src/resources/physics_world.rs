use crate::{Collider, Hit, QueryExclusions, RigidBody};

pub use {
    kodanu_ecs::{Query, Read, Res, ResMut, Resource, Write},
    kodanu_math::{Quat, Vec3},
    kodanu_transform::Transform,
};

use rapier3d::prelude::{
    Collider as RapierCollider, ColliderHandle as RapierColliderHandle,
    PhysicsWorld as RapierPhysicsWorld, Pose3 as RapierPose3, RigidBodyType as RapierRigidBodyType,
};

use rapier3d::{
    math::{Rot3 as RapierRot3, Vec3 as RapierVec3, Vector as RapierVector},
    pipeline::QueryFilter as RapierQueryFilter,
};

use rapier3d::parry::{
    query::{Ray as RapierRay, ShapeCastOptions as RapierShapeCastOptions},
    shape::Ball as RapierBall,
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

    pub fn raycast(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        exclude_colliders: Option<&[&Collider]>,
        exclude_rigid_bodies: Option<&[&RigidBody]>,
    ) -> Option<Hit> {
        if !max_distance.is_finite()
            || max_distance <= 0.0
            || !origin.is_finite()
            || !direction.is_finite()
        {
            return None;
        }

        let direction = Self::normalized_direction(direction)?;
        let origin = RapierVector::new(origin.x, origin.y, origin.z);
        let ray = RapierRay::new(origin, direction);

        let exclusions = QueryExclusions {
            colliders: exclude_colliders.unwrap_or_default(),
            rigid_bodies: exclude_rigid_bodies.unwrap_or_default(),
        };

        let predicate = |handle, collider: &RapierCollider| exclusions.allows(handle, collider);

        let filter = RapierQueryFilter {
            predicate: Some(&predicate),
            ..Default::default()
        };

        let query = self.physics.broad_phase.as_query_pipeline(
            self.physics.narrow_phase.query_dispatcher(),
            &self.physics.bodies,
            &self.physics.colliders,
            filter,
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

    pub fn spherecast(
        &self,
        origin: Vec3,
        radius: f32,
        direction: Vec3,
        max_distance: f32,
        exclude_colliders: Option<&[&Collider]>,
        exclude_rigid_bodies: Option<&[&RigidBody]>,
    ) -> Option<Hit> {
        if !origin.is_finite()
            || !direction.is_finite()
            || !radius.is_finite()
            || radius <= 0.0
            || !max_distance.is_finite()
            || max_distance <= 0.0
        {
            return None;
        }

        let direction = Self::normalized_direction(direction)?;
        let shape = RapierBall::new(radius);

        let shape_position = RapierPose3::from_parts(
            RapierVector::new(origin.x, origin.y, origin.z),
            RapierRot3::from_xyzw(0.0, 0.0, 0.0, 1.0),
        );

        let exclusions = QueryExclusions {
            colliders: exclude_colliders.unwrap_or_default(),
            rigid_bodies: exclude_rigid_bodies.unwrap_or_default(),
        };

        let predicate = |handle, collider: &RapierCollider| exclusions.allows(handle, collider);

        let filter = RapierQueryFilter {
            predicate: Some(&predicate),
            ..Default::default()
        };

        let query = self.physics.broad_phase.as_query_pipeline(
            self.physics.narrow_phase.query_dispatcher(),
            &self.physics.bodies,
            &self.physics.colliders,
            filter,
        );

        let (_, intersection) = query.cast_shape(
            &shape_position,
            direction,
            &shape,
            RapierShapeCastOptions {
                max_time_of_impact: max_distance,
                ..Default::default()
            },
        )?;

        Some(Hit::new(
            Vec3::new(
                intersection.witness1.x,
                intersection.witness1.y,
                intersection.witness1.z,
            ),
            Vec3::new(
                intersection.normal1.x,
                intersection.normal1.y,
                intersection.normal1.z,
            ),
            intersection.time_of_impact,
        ))
    }
}

impl PhysicsWorld {
    fn normalized_direction(direction: Vec3) -> Option<RapierVector> {
        if !direction.is_finite() {
            return None;
        }

        let length_squared = direction.length_squared();

        if length_squared <= f32::EPSILON {
            return None;
        }

        let inv_length = length_squared.sqrt().recip();

        Some(RapierVector::new(
            direction.x * inv_length,
            direction.y * inv_length,
            direction.z * inv_length,
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
