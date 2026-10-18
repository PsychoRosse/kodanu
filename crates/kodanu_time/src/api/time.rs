use {
    kodanu_ecs::Resource,
    kodanu_macros::Getters,
    serde::{Deserialize, Serialize},
};

/// Provides timing information for the engine.
///
/// `Time` is an ECS resource containing the current frame duration,
/// the elapsed time since the time driver was initialized, and the
/// maximum frame duration applied to [`Self::delta`].
///
/// The internal time driver measures real time, while `Time` exposes
/// the timing values to engine systems. The resource can be serialized
/// independently of the driver's platform-specific clock state.
///
/// Time values returned by [`Self::delta`] and [`Self::elapsed`]
/// are expressed in seconds.
#[derive(Getters, Resource, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Time {
    /// Returns the duration of the current frame in seconds.
    ///
    /// The returned value is clamped to [`Self::max_delta`].
    #[getter(copy, must_use)]
    delta: f32,
    /// Returns the total elapsed time in seconds.
    ///
    /// This value is measured by the time driver and is not affected
    /// by [`Self::max_delta`].
    #[getter(copy, must_use)]
    elapsed: f32,
    /// Returns the maximum duration that can be reported by [`Self::delta`],
    /// in seconds.
    #[getter(copy, must_use)]
    max_delta: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
            max_delta: Self::DEFAULT_MAX_DELTA,
        }
    }
}

impl Time {
    /// The default maximum frame duration.
    ///
    /// Defaults to `200 ms`.
    pub const DEFAULT_MAX_DELTA: f32 = 0.2;
}

impl Time {
    /// Updates the frame duration and elapsed time.
    ///
    /// The frame duration is clamped to [`Self::max_delta`].
    /// The elapsed time is not clamped and represents the total time
    /// reported by the time driver.
    pub(crate) fn update(&mut self, raw_delta: f32, elapsed: f32) {
        self.delta = raw_delta.min(self.max_delta);
        self.elapsed = elapsed;
    }
}

impl Time {
    /// Sets the maximum duration in seconds that can be reported by
    /// [`Self::delta`].
    ///
    /// This can prevent unusually large frame durations from destabilizing
    /// simulation after debugger pauses, window dragging, or temporary stalls.
    ///
    /// Setting this value to `0.0` causes [`Self::delta`] to
    /// return zero for every frame.
    pub fn set_max_delta(&mut self, max_delta: f32) {
        self.max_delta = max_delta;
        self.delta = self.delta.min(max_delta);
    }
}
