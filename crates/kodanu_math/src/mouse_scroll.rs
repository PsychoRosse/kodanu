use crate::MouseScrollDelta;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MouseScroll {
    Lines(MouseScrollDelta),
    Pixels(MouseScrollDelta),
}
