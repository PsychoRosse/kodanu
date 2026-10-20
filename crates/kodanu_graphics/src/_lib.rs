mod render;

pub mod prelude {
    pub use crate::{AssetServer, Backend, Color, Material, Mesh, MeshRenderer, Vertex};
}

pub use render::{api::*, components::*, plugins::*, resources::*};

pub(crate) use render::internal::*;
