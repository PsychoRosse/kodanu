//! Development and demo plugins for the Kodanu engine.
//!
//! This crate provides [`crate::DevPlugins`], which registers the standard
//! development stack, along with [`crate::DemoScenePlugin`] and
//! [`crate::FreeCameraPlugin`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod components;
mod plugins;

/// Commonly used types from `kodanu_plugins`.
///
/// ```
/// use kodanu_plugins::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{DemoScenePlugin, DevPlugins};
}

pub use {components::*, plugins::*};
