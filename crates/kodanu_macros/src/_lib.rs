//! Utility macros for the Kodanu engine.
//!
//! This crate provides [`map_with_capacity!`], a macro that creates a
//! `HashMap` with a pre-allocated capacity and inserts the given
//! key-value pairs.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod map_with_capacity;

/// Commonly used macros from `kodanu_macros`.
///
/// ```
/// use kodanu_macros::prelude::*;
/// // or explicitly: `use kodanu_macros::map_with_capacity;`
/// ```
pub mod prelude {
    pub use crate::map_with_capacity;
}
