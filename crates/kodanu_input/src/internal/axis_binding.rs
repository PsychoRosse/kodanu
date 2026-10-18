use {kodanu_macros::Getters, kodanu_window::Key};

#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct AxisBinding {
    #[getter(copy, must_use)]
    positive: Key,
    #[getter(copy, must_use)]
    negative: Key,
}

impl AxisBinding {
    pub fn new(positive: Key, negative: Key) -> Self {
        Self { positive, negative }
    }
}
