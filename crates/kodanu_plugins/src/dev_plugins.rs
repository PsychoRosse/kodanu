pub use {
    kodanu_ecs::{Plugin, Registry},
    kodanu_graphics::RenderPlugin,
    kodanu_input::InputPlugin,
    kodanu_physics::PhysicsPlugin,
    kodanu_time::TimePlugin,
    kodanu_window::WindowEventPlugin,
};

pub struct DevPlugins;

impl Plugin for DevPlugins {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_plugin(WindowEventPlugin)
            .with_plugin(InputPlugin)
            .with_plugin(TimePlugin)
            .with_plugin(RenderPlugin)
            .with_plugin(PhysicsPlugin);
    }
}
