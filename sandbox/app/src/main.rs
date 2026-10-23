use kodanu::prelude::*;

use kodanu_player::{DemoScenePlugin, PlayerPlugin};

fn main() -> Result<(), EventLoopError> {
    App::default()
        .with_plugin(DevPlugins)
        .with_plugin(PlayerPlugin)
        .with_plugin(DemoScenePlugin)
        .run_from(WindowAttributes::default())
}
