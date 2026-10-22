use crate::Material;

pub use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct MaterialUniform {
    color: [f32; 4],
}

impl From<&Material> for MaterialUniform {
    #[inline]
    fn from(value: &Material) -> Self {
        Self {
            color: value.color().get(),
        }
    }
}
