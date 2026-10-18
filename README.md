# Kodanu

Kodanu is a 3D game engine built on [wgpu](https://github.com/gfx-rs/wgpu), [winit](https://github.com/rust-windowing/winit), and an ECS core. Plugins register systems against a shared world, and the application drives them from a window event loop.

## Graphics Features

`kodanu_graphics` provides the rendering stack:

* Forward rendering with `wgpu` backends
* Perspective cameras and view-projection matrices
* Directional lights
* Meshes, vertices, and materials
* GPU instancing for repeated meshes
* Depth testing and MSAA views
* Asset storage with typed handles (`AssetServer`, `Handle`)
* Retained-mode UI components (rects, layout, styles, draw lists)

## Engine Features

* Plugin-based architecture (`App` + `Plugin`)
* ECS world, systems, and events
* Window events forwarded as ECS events
* Input, time, and fixed/variable timestep scheduling
* Physics via `rapier3d`
* Free camera and demo scene plugins

## Usage

The binary in `bin/` runs the standard development stack:

```rust
use kodanu::prelude::*;

fn main() {
    App::default()
        .with_plugin(DevPlugins)
        .run_from(WindowAttributes::default())
}
```

`DevPlugins` registers the window, input, time, render, and physics plugins, then spawns a demo scene with a free camera.

Run it with:

```sh
cargo run
```

## Workspace

| Crate | Purpose |
| --- | --- |
| `kodanu` | Umbrella crate and prelude |
| `kodanu_app` | Application entry point and event loop |
| `kodanu_ecs` | ECS world, systems, plugins |
| `kodanu_graphics` | wgpu renderer and UI components |
| `kodanu_camera` | Camera and projection components |
| `kodanu_input` | Input state and events |
| `kodanu_window` | Window event integration |
| `kodanu_time` | Time and scheduling |
| `kodanu_physics` | `rapier3d` integration |
| `kodanu_math` | Vectors, matrices, quaternions |
| `kodanu_transform` | Transform components |
| `kodanu_ai` | Machine learning integration |
| `kodanu_macros` | Derive macros |
| `kodanu_plugins` | Dev, demo, and free camera plugins |

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
