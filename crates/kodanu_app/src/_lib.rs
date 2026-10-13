//! Application entry point for the Kodanu game engine.
//!
//! This crate provides [`App`], which registers plugins, drives the
//! scheduler, and forwards window events into the ECS world.
//!
//! # Usage
//!
//! Configure the application with plugins and run it with the desired
//! window attributes:
//!
//! ```no_run
//! use {kodanu_app::prelude::*, kodanu_window::WindowAttributes};
//!
//! App::default().run_from(WindowAttributes::default());
//! ```
//!
//! Commonly used types can also be imported through [`prelude`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod app;

/// Commonly used types from `kodanu_app`.
///
/// ```
/// use kodanu_app::prelude::*;
/// ```
pub mod prelude {
    pub use crate::App;
}

pub use app::*;
