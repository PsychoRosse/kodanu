# kodanu_transform

Transform components for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_transform` provides [`Transform`](https://docs.rs/kodanu_transform/latest/kodanu_transform/struct.Transform.html), an ECS component used to describe the position, rotation, and scale of entities in 3D space, and [`Space`](https://docs.rs/kodanu_transform/latest/kodanu_transform/enum.Space.html), which selects the space a transformation is applied in.

## Features

* Position, rotation, and scale
* Validating constructor with rotation normalization
* Transformation matrices
* Inverse transformation matrices
* Local and world-space translation
* Local and world-space rotation
* Forward, backward, right, left, up, and down directions
* Transforming points from local space to world space
* ECS component integration

## Usage

Create a transform:

```rust
use {kodanu_math::{Quat, Vec3}, kodanu_transform::{Space, Transform}};

let transform = Transform::new(
    Vec3::new(10.0, 0.0, 5.0),
    Quat::IDENTITY,
    Vec3::ONE,
);
```

For common cases, use the convenience constructors:

```rust
let transform = Transform::from_position(Vec3::new(10.0, 0.0, 5.0));
```

## Transform Operations

`Transform` supports both world-space and local-space operations, selected with a `Space` argument.

```rust
// World space
transform.translate(Vec3::new(1.0, 0.0, 0.0), Space::Global);
transform.rotate(Vec3::UP, angle, Space::Global);

// Local space
transform.translate(Vec3::FORWARD, Space::Local);
transform.rotate(Vec3::RIGHT, angle, Space::Local);
```

`Space::Local` operations use the transform's current rotation, `Space::Global` operations do not.

## Directions

The transform provides its local basis directions in world space:

```rust
let forward = transform.forward();
let backward = transform.backward();
let right = transform.right();
let left = transform.left();
let up = transform.up();
let down = transform.down();
```

These are useful for movement, cameras, physics, and gameplay systems.

## Matrices

A transformation matrix can be obtained with:

```rust
let matrix = transform.matrix();
```

Points can be transformed from local space into world space:

```rust
let world_point = transform.transform_point(local_point);
```

The inverse transformation is available through the inverse matrix API.

## ECS

`Transform` implements `kodanu_ecs::Component` and can therefore be attached to entities and accessed from ECS systems.

```rust
fn movement_system(mut transforms: Query<Write<Transform>>) {
    for mut transform in transforms {
        transform.translate(Vec3::FORWARD, Space::Global);
    }
}
```

## License

Licensed under either:

* MIT License
* Apache License, Version 2.0

at your option.
