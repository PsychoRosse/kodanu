# kodanu_app

Application entry point for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_app` provides [`App`](https://docs.rs/kodanu_app/latest/kodanu_app/struct.App.html), which registers plugins, drives the scheduler, and forwards window events into the ECS world.

## Features

* Plugin registration and building
* Window event loop
* Startup system execution
* Fixed and variable timestep scheduling
* Window events forwarded as ECS events
* Graphics instance initialization
* ECS integration

## Usage

Add `kodanu_app` to your dependencies:

```toml
[dependencies]
kodanu_app = "0.1.0"
```

Import the commonly used types through the prelude:

```rust
use kodanu_app::prelude::*;
```

Build the application with plugins and run it:

```rust
App::default()
    .with_plugin(DevPlugins)
    .run_from(WindowAttributes::default());
```

`run_from` builds the registered plugins against the ECS world and the scheduler, then starts the window event loop. It returns only when the window is closed.

## Startup

When the window is created, the application initializes the graphics instance and runs the systems registered in the startup stage. Frame systems run afterwards on every redraw request.

## Events

Window events are forwarded into the ECS world as events:

| Window event | ECS event |
| --- | --- |
| `CloseRequested` | `Closing` |
| `RedrawRequested` | `Redrawing` |
| `SurfaceResized` | `SurfaceSize` |
| `KeyboardInput` | `KeyboardEvent` |
| `MouseInput` | `MouseEvent` |
| `PointerMoved` | `MousePos` |
| `MouseWheel` | `MouseWheelEvent` |

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
