use std::{marker::PhantomData, num::NonZeroU32};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Handle<A> {
    index: u32,
    gens: NonZeroU32,
    marker: PhantomData<fn() -> A>,
}

impl<A> Clone for Handle<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A> Copy for Handle<A> {}

impl<A> Handle<A> {
    pub(crate) const fn new(index: u32, gens: NonZeroU32) -> Self {
        Self {
            index,
            gens,
            marker: PhantomData,
        }
    }
}

impl<A> Handle<A> {
    pub const fn as_usize(&self) -> usize {
        self.index as usize
    }

    pub const fn index(&self) -> u32 {
        self.index
    }

    pub const fn gens(&self) -> NonZeroU32 {
        self.gens
    }
}
