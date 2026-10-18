//! Camera and projection components for the Kodanu game engine.
//!
//! This crate provides camera components, projection types, and camera-related
//! settings for rendering scenes in 3D.
//!
//! The main types are [`Camera`], [`Projection`], and
//! [`PerspectiveProjection`].
//!
//! # Examples
//!
//! ```
//! use kodanu_camera::prelude::*;
//!
//! let projection = PerspectiveProjection::new(90.0, 16.0 / 9.0, 0.03, 1000.0);
//! let camera = Camera::from(Projection::Perspective(projection));
//!
//! let projection_matrix = camera.projection_matrix();
//! ```
//!
//! Commonly used types can also be imported through [`prelude`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod api;
mod components;
mod resources;

/// Commonly used types from `kodanu_camera`.
///
/// ```
/// use kodanu_camera::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{ActiveCamera, Camera, CameraSettings, PerspectiveProjection, Projection};
}

pub use {api::*, components::*, resources::*};
