use {
    kodanu_ecs::Resource,
    kodanu_macros::Getters,
    serde::{Deserialize, Serialize},
};

#[derive(Getters, Resource, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Time {
    #[getter(copy, must_use)]
    delta: f32,
    #[getter(copy, must_use)]
    elapsed: f32,
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
    pub const DEFAULT_MAX_DELTA: f32 = 0.2;
}

impl Time {
    pub(crate) fn update(&mut self, raw_delta: f32, elapsed: f32) {
        self.delta = raw_delta.min(self.max_delta);
        self.elapsed = elapsed;
    }
}

impl Time {
    pub fn set_max_delta(&mut self, max_delta: f32) {
        self.max_delta = max_delta;
        self.delta = self.delta.min(max_delta);
    }
}
