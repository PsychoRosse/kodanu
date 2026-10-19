use crate::{DemoScenePlugin, PlayerPlugin};

pub use {
    kodanu_ecs::{Plugin, Registry},
    kodanu_graphics::RenderPlugin,
    kodanu_input::InputPlugin,
    kodanu_physics::PhysicsPlugin,
    kodanu_time::TimePlugin,
    kodanu_window::WindowEventPlugin,
};

/// Registers the standard development stack.
///
/// The plugin registers [`WindowEventPlugin`], [`InputPlugin`], [`TimePlugin`],
/// [`RenderPlugin`], and [`PhysicsPlugin`], followed by [`crate::DemoScenePlugin`]
/// and [`crate::FreeCameraPlugin`].
pub struct DevPlugins;

impl Plugin for DevPlugins {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_plugin(WindowEventPlugin)
            .with_plugin(InputPlugin)
            .with_plugin(TimePlugin)
            .with_plugin(RenderPlugin)
            .with_plugin(PhysicsPlugin)
            .with_plugin(DemoScenePlugin)
            .with_plugin(PlayerPlugin);
    }
}
