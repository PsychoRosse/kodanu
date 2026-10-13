# kodanu_camera

Camera and projection components for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_camera` provides camera components, projection types, and runtime settings for 3D rendering.

## Features

* Perspective projection
* Camera projection matrices
* View-projection matrix generation
* Active camera marker component
* Camera movement and sensitivity settings
* ECS integration

## Usage

Add `kodanu_camera` to your dependencies:

```toml
[dependencies]
kodanu_camera = "1.0.0"
```

Import the commonly used camera types through the prelude:

```rust
use kodanu_camera::prelude::*;
```

Create a camera with the default perspective projection:

```rust
let camera = Camera::default();
```

Or create one with a custom projection:

```rust
let projection = PerspectiveProjection::new(
    90.0,
    16.0 / 9.0,
    0.03,
    1000.0,
);

let camera = Camera::from(Projection::Perspective(projection));
```

## Projection

`PerspectiveProjection` describes a perspective camera using:

* vertical field of view in degrees
* viewport aspect ratio
* near clipping plane
* far clipping plane

The projection matrix can be obtained from a camera:

```rust
let projection_matrix = camera.projection_matrix();
```

## Active Camera

Use [`ActiveCamera`](https://docs.rs/kodanu_camera/latest/kodanu_camera/struct.ActiveCamera.html) to mark the entity that should be used as the active rendering camera:

```rust
commands.spawn((
    Camera::default(),
    ActiveCamera,
));
```

The component does not store any state itself. It acts as a marker that can be queried by rendering systems.

## Camera Settings

[`CameraSettings`](https://docs.rs/kodanu_camera/latest/kodanu_camera/struct.CameraSettings.html) stores runtime parameters used by camera controllers:

```rust
let settings = CameraSettings::new(100.0, 15.0);
```

The settings contain input sensitivity and camera movement speed.

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
