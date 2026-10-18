use {
    kodanu_ecs::Resource,
    std::time::{Duration, Instant},
};

/// Measures real time for the engine's [`crate::Time`] resource.
///
/// `TimeDriver` stores monotonic clock timestamps and is intentionally
/// kept separate from the serializable [`crate::Time`] resource.
///
/// The driver is initialized when it is created. Each update returns
/// the duration since the previous update and the total duration since
/// initialization.
#[derive(Resource, Debug)]
pub(crate) struct TimeDriver {
    startup: Instant,
    last: Instant,
}

impl Default for TimeDriver {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            startup: now,
            last: now,
        }
    }
}

impl TimeDriver {
    /// Measures the time since the previous update and since initialization.
    ///
    /// If either timestamp comparison cannot produce a duration,
    /// the corresponding value defaults to [`Duration::ZERO`].
    ///
    /// The returned frame duration is not clamped. Clamping is handled
    /// by [`crate::Time`].
    pub(crate) fn update(&mut self) -> (f32, f32) {
        let now = Instant::now();

        let delta = now
            .checked_duration_since(self.last)
            .unwrap_or(Duration::ZERO);

        let elapsed = now
            .checked_duration_since(self.startup)
            .unwrap_or(Duration::ZERO);

        self.last = now;

        (delta.as_secs_f32(), elapsed.as_secs_f32())
    }
}
