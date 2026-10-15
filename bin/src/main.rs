use kodanu::prelude::*;

fn main() {
    App::default()
        .with_plugin(DevPlugins)
        .run_from(WindowAttributes::default())
}
