use kodanu_window::{
    Closing, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing, WindowApplication,
    WindowAttributes, WindowEvent, WindowFrontend, WindowHandle, WindowId, WinitWindowApplication,
};

use {
    kodanu_ecs::{Plugin, PluginRegistry, Registry, Scheduler, World, WorldCell},
    kodanu_graphics::{Backend, Instance, InstanceDescriptor},
    kodanu_math::{MousePos, SurfaceSize},
    kodanu_plugins::DevPlugins,
    kodanu_time::Time,
};

use {pollster::block_on, std::process::exit};

/// The application entry point.
///
/// `App` bundles the plugin registry, the scheduler, and the ECS world.
/// Plugins are registered with [`Self::with_plugin`] and the application
/// is started with [`Self::run_from`].
#[derive(Default)]
pub struct App {
    plugins: PluginRegistry,
    schduler: Scheduler,
    world: World,
}

impl App {
    /// Builds the registered plugins and runs the application.
    ///
    /// `attr` describes the initial window attributes. The plugins are
    /// built against the ECS world and the scheduler, then the window
    /// event loop is started.
    ///
    /// This method does not return until the window is closed.
    pub fn run_from(mut self, attr: WindowAttributes) {
        self.plugins
            .build(WorldCell::from(&mut self.world), &mut self.schduler);

        WinitWindowApplication::new(attr, self).run();
    }
}

impl App {
    /// Registers a plugin and returns the application for chaining.
    ///
    /// Registered plugins are built by [`Self::run_from`].
    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.plugins.with_plugin(plugin);
        self
    }
}

impl WindowApplication for App {
    fn start(&mut self, window: &impl WindowFrontend, handle: &impl WindowHandle) {
        let descriptor =
            InstanceDescriptor::with(Backend::default(), handle.target().unwrap(), window.size());

        self.world.with_res(block_on(Instance::new(descriptor)));

        self.schduler.run_startup(WorldCell::from(&mut self.world));

        if !self.plugins.is_plugin_registered::<DevPlugins>() {
            exit(0)
        }
    }

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
}
