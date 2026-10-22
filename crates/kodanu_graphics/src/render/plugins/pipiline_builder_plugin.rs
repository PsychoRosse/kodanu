use crate::PipelineLayoutStorage;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct PipelineBuilderPlugin;

impl Plugin for PipelineBuilderPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(PipelineLayoutStorage::default())
            .with_system(
                Stage::Startup,
                PipelineLayoutStorage::pipeline_builder_startup_system,
            );
    }
}
