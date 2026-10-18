use crate::{
    Button, ButtonStatus, Key, KeyStatus, KeyboardEvent, MouseEvent, MouseWheelEvent, WindowEvent,
};

use winit::event::{
    ElementState as WinitElementState, MouseButton as WinitMouseButton,
    MouseScrollDelta as WinitMouseScrollDelta, WindowEvent as WinitWindowEvent,
};

use winit::keyboard::{KeyCode as WinitKeyCode, PhysicalKey as WinitPhysicalKey};

use kodanu_math::{MousePos, MouseScroll, MouseScrollDelta, SurfaceSize};

#[derive(Clone, Copy)]
pub struct WinitEventConverter;

impl WinitEventConverter {
    #[inline]
    pub fn from(event: WinitWindowEvent) -> Option<WindowEvent> {
        match event {
            WinitWindowEvent::CloseRequested => Some(WindowEvent::CloseRequested),
            WinitWindowEvent::RedrawRequested => Some(WindowEvent::RedrawRequested),
            WinitWindowEvent::SurfaceResized(size) => Some(WindowEvent::SurfaceResized(
                SurfaceSize::clamped(size.width, size.height),
            )),
            WinitWindowEvent::KeyboardInput { event, .. } => Some(WindowEvent::KeyboardInput(
                KeyboardEvent::new(Self::key(event.physical_key)?, Self::key_state(event.state)),
            )),
            WinitWindowEvent::PointerMoved { position, .. } => Some(WindowEvent::PointerMoved(
                MousePos::new(position.x as f32, position.y as f32),
            )),
            WinitWindowEvent::PointerButton { button, state, .. } => {
                Some(WindowEvent::MouseInput(MouseEvent::new(
                    Self::button(button.mouse_button()?)?,
                    Self::button_state(state),
                )))
            }
            WinitWindowEvent::MouseWheel { delta, .. } => match delta {
                WinitMouseScrollDelta::LineDelta(x, y) => Some(WindowEvent::MouseWheel(
                    MouseWheelEvent::from(MouseScroll::Lines(MouseScrollDelta::new(x, y))),
                )),
                WinitMouseScrollDelta::PixelDelta(delta) => {
                    Some(WindowEvent::MouseWheel(MouseWheelEvent::from(
                        MouseScroll::Pixels(MouseScrollDelta::new(delta.x as f32, delta.y as f32)),
                    )))
                }
            },
            _ => Some(WindowEvent::Unknown),
        }
    }

    #[inline]
    fn key_state(state: WinitElementState) -> KeyStatus {
        match state {
            WinitElementState::Pressed => KeyStatus::Pressed,
            WinitElementState::Released => KeyStatus::Released,
        }
    }

    #[inline]
    fn button_state(state: WinitElementState) -> ButtonStatus {
        match state {
            WinitElementState::Pressed => ButtonStatus::Pressed,
            WinitElementState::Released => ButtonStatus::Released,
        }
    }

    #[inline]
    fn key(key: WinitPhysicalKey) -> Option<Key> {
        match key {
            WinitPhysicalKey::Code(WinitKeyCode::KeyW) => Some(Key::W),
            WinitPhysicalKey::Code(WinitKeyCode::KeyA) => Some(Key::A),
            WinitPhysicalKey::Code(WinitKeyCode::KeyS) => Some(Key::S),
            WinitPhysicalKey::Code(WinitKeyCode::KeyD) => Some(Key::D),
            WinitPhysicalKey::Code(WinitKeyCode::KeyQ) => Some(Key::Q),
            WinitPhysicalKey::Code(WinitKeyCode::KeyE) => Some(Key::E),
            WinitPhysicalKey::Code(WinitKeyCode::ArrowUp) => Some(Key::UpArrow),
            WinitPhysicalKey::Code(WinitKeyCode::ArrowLeft) => Some(Key::LeftArrow),
            WinitPhysicalKey::Code(WinitKeyCode::ArrowDown) => Some(Key::DownArrow),
            WinitPhysicalKey::Code(WinitKeyCode::ArrowRight) => Some(Key::RightArrow),
            WinitPhysicalKey::Code(WinitKeyCode::Digit1) => Some(Key::One),
            WinitPhysicalKey::Code(WinitKeyCode::Digit2) => Some(Key::Two),
            WinitPhysicalKey::Code(WinitKeyCode::Digit3) => Some(Key::Three),
            WinitPhysicalKey::Code(WinitKeyCode::Digit4) => Some(Key::Four),
            WinitPhysicalKey::Code(WinitKeyCode::Digit5) => Some(Key::Five),
            WinitPhysicalKey::Code(WinitKeyCode::Digit6) => Some(Key::Six),
            WinitPhysicalKey::Code(WinitKeyCode::Digit7) => Some(Key::Seven),
            WinitPhysicalKey::Code(WinitKeyCode::Digit8) => Some(Key::Eight),
            WinitPhysicalKey::Code(WinitKeyCode::Digit9) => Some(Key::Nine),
            WinitPhysicalKey::Code(WinitKeyCode::Digit0) => Some(Key::Zero),
            WinitPhysicalKey::Code(WinitKeyCode::Escape) => Some(Key::Esc),
            WinitPhysicalKey::Code(WinitKeyCode::Tab) => Some(Key::Tab),
            WinitPhysicalKey::Code(WinitKeyCode::CapsLock) => Some(Key::Caps),
            WinitPhysicalKey::Code(WinitKeyCode::ShiftLeft) => Some(Key::LeftShift),
            WinitPhysicalKey::Code(WinitKeyCode::ShiftRight) => Some(Key::RightShift),
            WinitPhysicalKey::Code(WinitKeyCode::ControlLeft) => Some(Key::LeftCtrl),
            WinitPhysicalKey::Code(WinitKeyCode::ControlRight) => Some(Key::RightCtrl),
            WinitPhysicalKey::Code(WinitKeyCode::AltLeft) => Some(Key::LeftAlt),
            WinitPhysicalKey::Code(WinitKeyCode::AltRight) => Some(Key::RightAlt),
            WinitPhysicalKey::Code(WinitKeyCode::Space) => Some(Key::Space),
            WinitPhysicalKey::Code(WinitKeyCode::KeyH) => Some(Key::H),
            WinitPhysicalKey::Code(WinitKeyCode::KeyJ) => Some(Key::J),
            WinitPhysicalKey::Code(WinitKeyCode::KeyK) => Some(Key::K),
            WinitPhysicalKey::Code(WinitKeyCode::KeyL) => Some(Key::L),
            _ => None,
        }
    }

    #[inline]
    fn button(button: WinitMouseButton) -> Option<Button> {
        match button {
            WinitMouseButton::Left => Some(Button::Left),
            WinitMouseButton::Middle => Some(Button::Middle),
            WinitMouseButton::Right => Some(Button::Right),
            _ => None,
        }
    }
}
