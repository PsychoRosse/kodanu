#![allow(dead_code)]

use crate::{
    BindGroupLayout, CameraLayout, Instance, LightLayout, MaterialLayout, ModelLayout,
    PipelineLayoutStorageError, RenderPipeline,
};

use {
    kodanu_ecs::{Commands, Res, ResMut, Resource},
    std::any::TypeId,
};

use wgpu::BindGroupLayout as WgpuBindGroupLayout;

#[derive(Resource, Default)]
pub struct PipelineLayoutStorage {
    layouts: Vec<Box<dyn BindGroupLayout>>,
}

impl PipelineLayoutStorage {
    pub fn push<L: BindGroupLayout>(&mut self, layout: L) -> &mut Self {
        self.layouts.push(Box::new(layout));
        self
    }

    pub fn get<L: BindGroupLayout>(&self) -> Option<&L> {
        self.layouts
            .iter()
            .find(|layout| layout.as_any().type_id() == TypeId::of::<L>())
            .and_then(|layout| layout.as_any().downcast_ref::<L>())
    }

    pub fn get_now<L: BindGroupLayout>(&self) -> &WgpuBindGroupLayout {
        self.get::<L>()
            .unwrap_or_else(|| panic!("{}", PipelineLayoutStorageError::PipelineNotFound))
            .get()
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn BindGroupLayout> {
        self.layouts.iter().map(|boxed| boxed.as_ref())
    }

    pub fn get_all(&self) -> Vec<Option<&WgpuBindGroupLayout>> {
        let max_group = self
            .layouts
            .iter()
            .map(|layout| layout.group())
            .max()
            .unwrap_or(0);

        let mut layouts = vec![None; max_group as usize + 1];

        for layout in &self.layouts {
            layouts[layout.group() as usize] = Some(layout.get())
        }

        layouts
    }
}

impl PipelineLayoutStorage {
    pub fn pipeline_builder_startup_system(
        commands: Commands,
        instance: Res<Instance>,
        mut storage: ResMut<PipelineLayoutStorage>,
    ) {
        storage
            .push(CameraLayout::new(instance.device()))
            .push(MaterialLayout::new(instance.device()))
            .push(ModelLayout::new(instance.device()))
            .push(LightLayout::new(instance.device()));

        commands.with_res(RenderPipeline::new(
            instance.device(),
            instance.format(),
            &storage.get_all(),
        ));
    }
}
