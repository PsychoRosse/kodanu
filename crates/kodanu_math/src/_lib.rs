//! Core mathematical types and utilities for the Kodanu game engine.
//!
//! `kodanu_math` provides vectors, quaternions, matrices, and types related
//! to mouse input and window dimensions.
//!
//! # Main types
//!
//! - [`crate::Vec2`], [`crate::Vec3`], and [`crate::Vec4`] for vector arithmetic.
//! - [`crate::Quat`] for representing 3D rotations.
//! - [`crate::Mat4`] for 4 × 4 matrix operations and transformations.
//! - [`crate::MousePos`], [`crate::MouseDelta`], [`crate::MouseScroll`], and
//!   [`crate::MouseScrollDelta`] for mouse input data.
//! - [`crate::SurfaceSize`] and [`crate::WindowSize`] for surface and window dimensions.
//!
//! # Examples
//!
//! ```
//! use kodanu_math::{Mat4, Quat, Vec3};
//!
//! let position = Vec3::new(1.0, 2.0, 3.0);
//! let rotation = Quat::from_rotation_y(90.0);
//! let transform = Mat4::from_translation(position)
//!     * Mat4::from_quat(rotation);
//! ```
//!
//! Commonly used types can also be imported through [`prelude`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod mat4;
mod mouse_delta;
mod mouse_pos;
mod mouse_scroll;
mod mouse_scroll_delta;
mod quat;
mod surface_size;
mod vec2;
mod vec3;
mod vec4;
mod window_size;

/// Commonly used types from `kodanu_math`.
///
/// ```
/// use kodanu_math::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        Mat4, MouseDelta, MousePos, MouseScroll, MouseScrollDelta, Quat, SurfaceSize, Vec2, Vec3,
        Vec4, WindowSize,
    };
}

pub use {
    mat4::Mat4, mouse_delta::MouseDelta, mouse_pos::MousePos, mouse_scroll::MouseScroll,
    mouse_scroll_delta::MouseScrollDelta, quat::Quat, surface_size::SurfaceSize, vec2::Vec2,
    vec3::Vec3, vec4::Vec4, window_size::WindowSize,
};
