use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

use kodanu_math::{MousePos, SurfaceSize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum WindowEvent {
    CloseRequested,
    RedrawRequested,
    SurfaceResized(SurfaceSize),
    KeyboardInput(KeyboardEvent),
    MouseInput(MouseEvent),
    PointerMoved(MousePos),
    MouseWheel(MouseWheelEvent),
    Unknown,
}
