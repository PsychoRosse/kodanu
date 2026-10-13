use {
    kodanu_ecs::Resource,
    serde::{Deserialize, Serialize},
    std::time::Duration,
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
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    delta: Duration,
    elapsed: Duration,
    max_delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            max_delta: Self::DEFAULT_MAX_DELTA,
        }
    }
}

impl Time {
    /// The default maximum frame duration.
    ///
    /// Defaults to `200 ms`.
    pub const DEFAULT_MAX_DELTA: Duration = Duration::from_millis(200);
}

impl Time {
    /// Updates the frame duration and elapsed time.
    ///
    /// The frame duration is clamped to [`Self::max_delta`].
    /// The elapsed time is not clamped and represents the total time
    /// reported by the time driver.
    pub(crate) fn update(&mut self, raw_delta: Duration, elapsed: Duration) {
        self.delta = raw_delta.min(self.max_delta);
        self.elapsed = elapsed;
    }
}

impl Time {
    /// Returns the duration of the current frame in seconds.
    ///
    /// The returned value is clamped to [`Self::max_delta`].
    #[must_use]
    pub const fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Returns the total elapsed time in seconds.
    ///
    /// This value is measured by the time driver and is not affected
    /// by [`Self::max_delta`].
    #[must_use]
    pub const fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    /// Returns the maximum duration that can be reported by [`Self::delta`].
    #[must_use]
    pub const fn max_delta(&self) -> Duration {
        self.max_delta
    }

    /// Sets the maximum duration that can be reported by [`Self::delta`].
    ///
    /// This can prevent unusually large frame durations from destabilizing
    /// simulation after debugger pauses, window dragging, or temporary stalls.
    ///
    /// Setting this value to [`std::time::Duration::ZERO`] causes [`Self::delta`] to
    /// return zero for every frame.
    pub fn set_max_delta(&mut self, max_delta: Duration) {
        self.max_delta = max_delta;
        self.delta = self.delta.min(max_delta);
    }
}
