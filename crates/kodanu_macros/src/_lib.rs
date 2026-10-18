//! Utility macros for the Kodanu engine.
//!
//! This crate provides the [`Getters`] derive macro, which generates
//! read-only accessors for struct fields, and [`map_with_capacity!`], a macro
//! that creates a `HashMap` with a pre-allocated capacity and inserts the
//! given key-value pairs.
//!
//! # Examples
//!
//! ```
//! use {
//!     kodanu_macros::{Getters, map_with_capacity},
//!     std::collections::HashMap,
//! };
//!
//! #[derive(Getters)]
//! struct WindowId {
//!     #[getter(copy)]
//!     id: usize,
//! }
//!
//! let id = WindowId { id: 7 };
//! assert_eq!(id.id(), 7);
//!
//! let map: HashMap<&str, i32> = map_with_capacity!(2; "a" => 1, "b" => 2);
//! assert_eq!(map.len(), 2);
//! ```
//!
//! Commonly used macros can also be imported through [`prelude`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod map_with_capacity;

/// Commonly used macros from `kodanu_macros`.
///
/// ```
/// use kodanu_macros::prelude::*;
/// // or explicitly: `use kodanu_macros::{Getters, map_with_capacity};`
/// ```
pub mod prelude {
    pub use crate::{Getters, map_with_capacity};
}

pub use getters::Getters;
