mod render;
mod ui;

pub mod prelude {
    pub use crate::{AssetServer, Backend, Color, Material, Mesh, MeshRenderer, Rect, Vertex};
}

pub use {
    render::{api::*, components::*, plugins::*, resources::*},
    ui::components::*,
};

pub(crate) use render::internal::*;
