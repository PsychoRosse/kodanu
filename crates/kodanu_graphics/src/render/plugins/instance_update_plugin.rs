use crate::Instance;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct InstanceUpdatePlugin;

impl Plugin for InstanceUpdatePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::PreRender, Instance::instance_resize_system);
    }
}
