//! Time management for the Kodanu engine.
//!
//! This crate provides [`Time`], an ECS resource containing frame timing
//! information, and [`TimePlugin`], which registers the system responsible
//! for updating it.
//!
//! The internal time driver measures real time independently of the
//! serializable [`Time`] resource.
//!
//! # Usage
//!
//! Add [`TimePlugin`] to the engine to register the time resource and
//! update system. Systems can then access timing information through
//! `Res<Time>`.
//!
//! ```
//! use {kodanu_time::Time, kodanu_ecs::Res};
//!
//! fn movement_system(time: Res<Time>) {
//!     let delta = time.delta();
//!     // Update movement using the frame duration.
//! }
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod api;
mod internal;
mod plugins;

/// Commonly used types from `kodanu_time`.
///
/// ```
/// use kodanu_time::Time;
/// ```
pub mod prelude {
    pub use crate::{Time, TimePlugin};
}

pub use {api::*, plugins::*};

pub(crate) use internal::*;
