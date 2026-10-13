# kodanu_plugins

Development and demo plugins for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_plugins` provides [`DevPlugins`](https://docs.rs/kodanu_plugins/latest/kodanu_plugins/struct.DevPlugins.html), a bundle that registers the standard development stack, along with standalone demo scene and free camera plugins.

## Features

* Demo scene with physics-enabled cubes and lighting
* Free-flying camera controlled by input axes
* Bundled development plugin stack
* ECS plugin integration

## Usage

Add `kodanu_plugins` to your dependencies:

```toml
[dependencies]
kodanu_plugins = "0.1.0"
```

Register the development stack in the application:

```rust
use kodanu_plugins::prelude::*;

App::default()
    .with_plugin(DevPlugins)
    .run_from(WindowAttributes::default());
```

## DevPlugins

Registers the window event, input, time, render, and physics plugins, followed by the demo scene and the free camera.

## Demo Scene

Spawns a static platform, three dynamic cubes, and a directional light during startup. All entities are simulated with rigid bodies and colliders.

## Free Camera

Registers `CameraSettings`, spawns the active camera entity, and moves the camera from input axes during late update. Movement speed and look sensitivity come from the settings resource.

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
