use {
    kodanu_macros::Getters,
    serde::{Deserialize, Serialize},
};

use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::NonZeroU32,
};

#[derive(Getters, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AssetHandle<A> {
    #[getter(copy)]
    index: u32,
    #[getter(copy)]
    gens: NonZeroU32,
    marker: PhantomData<fn() -> A>,
}

impl<A> Clone for AssetHandle<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A> Copy for AssetHandle<A> {}

impl<A> PartialEq for AssetHandle<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.gens == other.gens
    }
}

impl<A> Eq for AssetHandle<A> {}

impl<A> Hash for AssetHandle<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.gens.hash(state);
    }
}

impl<A> AssetHandle<A> {
    pub(crate) const fn new(index: u32, gens: NonZeroU32) -> Self {
        Self {
            index,
            gens,
            marker: PhantomData,
        }
    }
}

impl<A> AssetHandle<A> {
    pub const fn as_usize(&self) -> usize {
        self.index as usize
    }
}
