use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

use kodanu_math::{MousePos, SurfaceSize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
