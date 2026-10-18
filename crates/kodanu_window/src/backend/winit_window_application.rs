use crate::{
    EventLoopError, WindowApplication, WindowAttributes, WindowError, WindowEvent, WindowFrontend,
    WindowId, WinitEventConverter, WinitWindowBackend,
};

use winit::{
    application::ApplicationHandler as WinitApplicationHandler,
    event::WindowEvent as WinitWindowEvent,
    event_loop::{ActiveEventLoop as WinitActiveEventLoop, EventLoop as WinitEventLoop},
    window::{WindowAttributes as WinitWindowAttributes, WindowId as WinitWindowId},
};

pub struct WinitWindowApplication<A: WindowApplication> {
    application: A,
    window: Option<WinitWindowBackend>,
    attributes: WindowAttributes,
}

impl<A: WindowApplication> WinitWindowApplication<A> {
    pub fn new(attributes: WindowAttributes, application: A) -> Self {
        Self {
            application,
            window: None,
            attributes,
        }
    }
}

impl<A: WindowApplication> WinitWindowApplication<A> {
    pub fn run(self) {
        let event_loop = WinitEventLoop::new()
            .unwrap_or_else(|_| panic!("{}", EventLoopError::EventLoopCreation));

        event_loop
            .run_app(self)
            .unwrap_or_else(|_| panic!("{}", EventLoopError::EventLoopRun));
    }
}

impl<A: WindowApplication> WinitApplicationHandler for WinitWindowApplication<A> {
    #[inline]
    fn window_event(
        &mut self,
        event_loop: &dyn WinitActiveEventLoop,
        window_id: WinitWindowId,
        window_event: WinitWindowEvent,
    ) {
        let Some(window) = &self.window else { return };

        let Some(event) = WinitEventConverter::from(window_event) else {
            return;
        };

        self.application
            .event(event, WindowId::new(window_id.into_raw()));

        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }

        window.redraw();
    }

    #[inline]
    fn about_to_wait(&mut self, _: &dyn WinitActiveEventLoop) {
        self.application.tick()
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn WinitActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = event_loop
            .create_window(WinitWindowAttributes::from(self.attributes))
            .unwrap_or_else(|_| panic!("{}", WindowError::WindowCreation));

        let window = WinitWindowBackend::from(window);

        self.application.start(&window, &window);

        self.window = Some(window)
    }

    fn resumed(&mut self, _: &dyn WinitActiveEventLoop) {}
}
