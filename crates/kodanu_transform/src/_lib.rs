//! Spatial transformations for entities in the Kodanu engine.
//!
//! This crate provides [`Transform`], a component that describes an entity's
//! position, rotation, and scale in 3D space, and [`Space`], which selects
//! the space a transformation is applied in.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod api;
mod components;

/// Commonly used types from `kodanu_transform`.
///
/// ```
/// use kodanu_transform::prelude::*;
/// // or explicitly: `use kodanu_transform::{Space, Transform, TransformError};`
/// ```
pub mod prelude {
    pub use crate::{Space, Transform, TransformError};
}

pub use {api::*, components::*};
