use kodanu::prelude::*;

fn main() {
    App::default()
        .with_plugin(AiPlugin)
        .run_from(WindowAttributes::default())
}
