use kodanu_window::{
    Application, Closing, EventLoopError, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing,
    WindowApplication, WindowAttributes, WindowEvent, WindowFrontend, WindowHandle, WindowId,
};

use {
    kodanu_ecs::{Plugin, PluginRegistry, Registry, Scheduler, World, WorldCell},
    kodanu_graphics::{Backend, Instance, InstanceDescriptor},
    kodanu_math::{MousePos, SurfaceSize},
    kodanu_plugins::DevPlugins,
    kodanu_time::Time,
};

use {pollster::block_on, std::process::exit};

#[derive(Default)]
pub struct App {
    plugins: PluginRegistry,
    schduler: Scheduler,
    world: World,
}

impl App {
    pub fn run_from(mut self, attr: WindowAttributes) -> Result<(), EventLoopError> {
        self.plugins
            .build(WorldCell::from(&mut self.world), &mut self.schduler);

        WindowApplication::new(attr, self).run()?;

        Ok(())
    }
}

impl App {
    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.plugins.with_plugin(plugin);
        self
    }
}

impl Application for App {
    #[inline]
    fn event(&mut self, event: WindowEvent, _: WindowId) {
        let cell = WorldCell::from(&mut self.world);

        match event {
            WindowEvent::CloseRequested => {
                cell.expect_mut_event::<Closing>().send(Closing);
            }
            WindowEvent::RedrawRequested => {
                cell.expect_mut_event::<Redrawing>().send(Redrawing);

                cell.update();

                self.schduler
                    .run_full(cell, cell.expect_res::<Time>().delta());
            }
            WindowEvent::SurfaceResized(size) => {
                cell.expect_mut_event::<SurfaceSize>().send(size);
            }
            WindowEvent::KeyboardInput(event) => {
                cell.expect_mut_event::<KeyboardEvent>().send(event);
            }
            WindowEvent::MouseInput(event) => {
                cell.expect_mut_event::<MouseEvent>().send(event);
            }
            WindowEvent::PointerMoved(position) => {
                cell.expect_mut_event::<MousePos>().send(position);
            }
            WindowEvent::MouseWheel(delta) => {
                cell.expect_mut_event::<MouseWheelEvent>().send(delta);
            }
            _ => {}
        }
    }

    fn start(&mut self, window: &impl WindowFrontend, handle: &impl WindowHandle) {
        let descriptor =
            InstanceDescriptor::with(Backend::default(), handle.target().unwrap(), window.size());

        self.world.with_res(block_on(Instance::new(descriptor)));

        self.schduler.run_startup(WorldCell::from(&mut self.world));

        if !self.plugins.is_plugin_registered::<DevPlugins>() {
            exit(0)
        }
    }
}
