use kodanu_macros::Getters;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Getters, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(#[getter(copy, name = "get")] usize);

impl WindowId {
    #[inline]
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
