//! Spatial transformations for entities in the Kodanu engine.
//!
//! This crate provides [`crate::Transform`], a component that describes an entity's
//! position, rotation, and scale in 3D space, and [`crate::Space`], which selects
//! the space a transformation is applied in.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod space;
mod transform;
mod transform_error;

/// Commonly used types from `kodanu_transform`.
///
/// ```
/// use kodanu_transform::prelude::*;
/// // or explicitly: `use kodanu_transform::{Space, Transform, TransformError};`
/// ```
pub mod prelude {
    pub use crate::{Space, Transform, TransformError};
}

pub use {space::Space, transform::Transform, transform_error::TransformError};
