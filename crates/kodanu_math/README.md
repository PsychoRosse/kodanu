# kodanu_math

Mathematics primitives and operations for the kodanu game engine.

`kodanu_math` provides foundational mathematical types and operations used in 2D and 3D graphics, transformations, and game development.

## Features

* **Vectors:** `Vec2`, `Vec3`, and `Vec4`
* **Quaternions:** `Quat` for representing 3D rotations
* **Matrices:** `Mat4` for transformations and projection
* **Mouse input types:** `MousePos`, `MouseDelta`, `MouseScroll`, and `MouseScrollDelta`
* **Surface and window dimensions:** `SurfaceSize` and `WindowSize`
* Vector arithmetic and common geometric operations
* Rotation, scaling, translation, and perspective projection utilities

## Usage

Add `kodanu_math` to your dependencies:

```toml
[dependencies]
kodanu_math = "1.0.0"
```

Import the commonly used types through the prelude:

```rust
use kodanu_math::prelude::*;
```

Create a position and rotate a direction vector:

```rust
let position = Vec3::new(1.0, 2.0, 3.0);
let rotation = Quat::from_rotation_y(90.0);
let direction = rotation * Vec3::FORWARD;

println!("Position: {position:?}");
println!("Direction: {direction:?}");
```

## Documentation

Generate the API documentation locally:

```bash
cargo doc -p kodanu_math --no-deps --open
```

Run the crate's tests:

```bash
cargo test -p kodanu_math
```

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
