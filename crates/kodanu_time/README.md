# kodanu_time

Time management and frame timing for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_time` provides [`Time`](https://docs.rs/kodanu_time/latest/kodanu_time/struct.Time.html), the ECS resource that carries frame timing information, and [`TimePlugin`](https://docs.rs/kodanu_time/latest/kodanu_time/struct.TimePlugin.html), which registers the resource and the system that refreshes it.

## Features

* Frame delta time
* Elapsed time since startup
* Delta-time clamping
* Serializable time resource
* ECS integration

## Usage

Add `kodanu_time` to your dependencies:

```toml
[dependencies]
kodanu_time = "1.0.0"
```

Register the plugin in the application:

```rust
App::default()
    .with_plugin(TimePlugin)
    .run_from(WindowAttributes::default());
```

Import the commonly used types through the prelude:

```rust
use kodanu_time::prelude::*;
```

Systems access the time resource through the ECS:

```rust
use kodanu_ecs::Res;
use kodanu_time::Time;

fn update(time: Res<Time>) {
    let delta = time.delta();

    // Update game state.
    let _ = delta;
}
```

`delta()`, `elapsed()`, and `max_delta()` return time in seconds as `f32`.

## Delta Clamping

Frame durations are clamped to `max_delta()`, which defaults to 200 ms. This keeps unusually long frames, such as debugger pauses or window dragging, from destabilizing the simulation:

```rust
time.set_max_delta(0.1);
```

Setting the maximum to `0.0` makes `delta()` return zero for every frame.

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

at your option.
