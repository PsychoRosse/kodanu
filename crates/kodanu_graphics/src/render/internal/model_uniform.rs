use {
    bytemuck::{Pod, Zeroable},
    kodanu_math::Mat4,
};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct ModelUniform {
    model: [[f32; 4]; 4],
    normal: [[f32; 4]; 4],
}

impl From<Mat4> for ModelUniform {
    #[inline]
    fn from(value: Mat4) -> Self {
        Self {
            model: value.to_cols_array_2d(),
            normal: value.inverse().transpose().to_cols_array_2d(),
        }
    }
}
