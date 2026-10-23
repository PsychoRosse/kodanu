use kodanu::prelude::*;

fn main() -> Result<(), EventLoopError> {
    App::default()
        .with_plugin(DevPlugins)
        .run_from(WindowAttributes::default())
}
