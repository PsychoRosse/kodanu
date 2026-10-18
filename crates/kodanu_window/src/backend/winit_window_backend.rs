use crate::{SurfaceTarget, WindowFrontend, WindowHandle, WindowHandlerError};

use {
    kodanu_math::SurfaceSize,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    std::sync::Arc,
};

use winit::window::Window as WinitWindow;

pub struct WinitWindowBackend {
    window: Arc<dyn WinitWindow>,
}

impl WindowFrontend for WinitWindowBackend {
    #[inline]
    fn redraw(&self) {
        self.window.request_redraw();
    }

    fn size(&self) -> SurfaceSize {
        let size = self.window.surface_size();

        SurfaceSize::clamped(size.width, size.height)
    }
}

impl WindowHandle for WinitWindowBackend {
    fn target(&self) -> Result<SurfaceTarget, WindowHandlerError> {
        let window = self
            .window
            .display_handle()
            .map_err(|_| WindowHandlerError::RawDisplayHandle)?
            .as_raw();

        let display = self
            .window
            .window_handle()
            .map_err(|_| WindowHandlerError::RawWindowHandle)?
            .as_raw();

        Ok(SurfaceTarget::new(window, display))
    }
}

impl From<Box<dyn WinitWindow>> for WinitWindowBackend {
    fn from(value: Box<dyn WinitWindow>) -> Self {
        Self {
            window: Arc::from(value),
        }
    }
}
