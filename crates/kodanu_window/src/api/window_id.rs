use kodanu_macros::Getters;

#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(#[getter(name = "get", copy)] usize);

impl WindowId {
    #[inline]
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
