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
