pub mod prelude {
    pub use kodanu_app::prelude::*;
    pub use kodanu_camera::prelude::*;
    pub use kodanu_ecs::prelude::*;
    pub use kodanu_graphics::prelude::*;
    pub use kodanu_input::prelude::*;
    pub use kodanu_macros::prelude::*;
    pub use kodanu_math::prelude::*;
    pub use kodanu_plugins::prelude::*;
    pub use kodanu_time::prelude::*;
    pub use kodanu_transform::prelude::*;
    pub use kodanu_window::prelude::*;

    #[cfg(feature = "physics")]
    pub use kodanu_physics::prelude::*;
}

#[cfg(feature = "physics")]
pub use kodanu_physics as physics;

pub use {
    kodanu_app as app, kodanu_camera as camera, kodanu_ecs as ecs, kodanu_graphics as graphics,
    kodanu_input as input, kodanu_math as math, kodanu_plugins as plugins, kodanu_time as time,
    kodanu_transform as transform, kodanu_window as window,
};
