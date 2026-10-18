use crate::{Time, TimeDriver};

use kodanu_ecs::{Plugin, Registry, ResMut, Stage};

/// Registers the [`crate::Time`] resource and its update system.
///
/// The plugin initializes the public time resource and the internal
/// time driver. It registers `time_system` in [`Stage::TimeUpdate`],
/// where the frame duration and elapsed time are refreshed.
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(Time::default())
            .with_res(TimeDriver::default())
            .with_system(Stage::TimeUpdate, time_system);
    }
}

/// Updates [`crate::Time`] using measurements from [`crate::TimeDriver`].
///
/// This system should run before systems that consume [`crate::Time`], so they
/// observe the current frame's timing values.
fn time_system(mut time: ResMut<Time>, mut driver: ResMut<TimeDriver>) {
    let (delta, elapsed) = driver.update();

    time.update(delta, elapsed);
}
