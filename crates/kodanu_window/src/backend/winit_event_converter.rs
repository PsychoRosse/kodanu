use crate::winit;

use crate::{
    Button, ButtonStatus, Key, KeyStatus, KeyboardEvent, MouseEvent, MouseWheelEvent, WindowEvent,
};

use kodanu_math::{MousePos, MouseScroll, MouseScrollDelta, SurfaceSize};

#[derive(Clone, Copy)]
pub struct WinitEventConverter;

impl WinitEventConverter {
    #[inline]
    pub fn from(event: winit::WindowEvent) -> Option<WindowEvent> {
        match event {
            winit::WindowEvent::CloseRequested => {
                return Some(WindowEvent::CloseRequested);
            }
            winit::WindowEvent::RedrawRequested => {
                return Some(WindowEvent::RedrawRequested);
            }
            winit::WindowEvent::SurfaceResized(size) => {
                let size = SurfaceSize::clamped(size.width, size.height);

                Some(WindowEvent::SurfaceResized(size))
            }
            winit::WindowEvent::KeyboardInput { event, .. } => {
                let event = KeyboardEvent::new(
                    Self::key(event.physical_key)?,
                    Self::key_state(event.state),
                );

                Some(WindowEvent::KeyboardInput(event))
            }
            winit::WindowEvent::PointerMoved { position, .. } => {
                let pos = MousePos::new(position.x as f32, position.y as f32);

                Some(WindowEvent::PointerMoved(pos))
            }
            winit::WindowEvent::PointerButton { button, state, .. } => {
                let event = MouseEvent::new(
                    Self::button(button.mouse_button()?)?,
                    Self::button_state(state),
                );

                Some(WindowEvent::MouseInput(event))
            }
            winit::WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::MouseScrollDelta::LineDelta(x, y) => {
                    let delta = MouseScrollDelta::new(x, y);
                    let event = MouseWheelEvent::from(MouseScroll::Lines(delta));

                    Some(WindowEvent::MouseWheel(event))
                }
                winit::MouseScrollDelta::PixelDelta(delta) => {
                    let delta = MouseScrollDelta::new(delta.x as f32, delta.y as f32);
                    let event = MouseWheelEvent::from(MouseScroll::Pixels(delta));

                    Some(WindowEvent::MouseWheel(event))
                }
                _ => None,
            },
            _ => Some(WindowEvent::Unknown),
        }
    }

    #[inline]
    fn key_state(state: winit::ElementState) -> KeyStatus {
        match state {
            winit::ElementState::Pressed => KeyStatus::Pressed,
            winit::ElementState::Released => KeyStatus::Released,
        }
    }

    #[inline]
    fn button_state(state: winit::ElementState) -> ButtonStatus {
        match state {
            winit::ElementState::Pressed => ButtonStatus::Pressed,
            winit::ElementState::Released => ButtonStatus::Released,
        }
    }

    #[inline]
    fn key(key: winit::PhysicalKey) -> Option<Key> {
        match key {
            winit::PhysicalKey::Code(winit::KeyCode::KeyW) => Some(Key::W),
            winit::PhysicalKey::Code(winit::KeyCode::KeyA) => Some(Key::A),
            winit::PhysicalKey::Code(winit::KeyCode::KeyS) => Some(Key::S),
            winit::PhysicalKey::Code(winit::KeyCode::KeyD) => Some(Key::D),
            winit::PhysicalKey::Code(winit::KeyCode::KeyQ) => Some(Key::Q),
            winit::PhysicalKey::Code(winit::KeyCode::KeyE) => Some(Key::E),
            winit::PhysicalKey::Code(winit::KeyCode::ArrowUp) => Some(Key::UpArrow),
            winit::PhysicalKey::Code(winit::KeyCode::ArrowLeft) => Some(Key::LeftArrow),
            winit::PhysicalKey::Code(winit::KeyCode::ArrowDown) => Some(Key::DownArrow),
            winit::PhysicalKey::Code(winit::KeyCode::ArrowRight) => Some(Key::RightArrow),
            winit::PhysicalKey::Code(winit::KeyCode::Digit1) => Some(Key::One),
            winit::PhysicalKey::Code(winit::KeyCode::Digit2) => Some(Key::Two),
            winit::PhysicalKey::Code(winit::KeyCode::Digit3) => Some(Key::Three),
            winit::PhysicalKey::Code(winit::KeyCode::Digit4) => Some(Key::Four),
            winit::PhysicalKey::Code(winit::KeyCode::Digit5) => Some(Key::Five),
            winit::PhysicalKey::Code(winit::KeyCode::Digit6) => Some(Key::Six),
            winit::PhysicalKey::Code(winit::KeyCode::Digit7) => Some(Key::Seven),
            winit::PhysicalKey::Code(winit::KeyCode::Digit8) => Some(Key::Eight),
            winit::PhysicalKey::Code(winit::KeyCode::Digit9) => Some(Key::Nine),
            winit::PhysicalKey::Code(winit::KeyCode::Digit0) => Some(Key::Zero),
            winit::PhysicalKey::Code(winit::KeyCode::Escape) => Some(Key::Esc),
            winit::PhysicalKey::Code(winit::KeyCode::Tab) => Some(Key::Tab),
            winit::PhysicalKey::Code(winit::KeyCode::CapsLock) => Some(Key::Caps),
            winit::PhysicalKey::Code(winit::KeyCode::ShiftLeft) => Some(Key::LeftShift),
            winit::PhysicalKey::Code(winit::KeyCode::ShiftRight) => Some(Key::RightShift),
            winit::PhysicalKey::Code(winit::KeyCode::ControlLeft) => Some(Key::LeftCtrl),
            winit::PhysicalKey::Code(winit::KeyCode::ControlRight) => Some(Key::RightCtrl),
            winit::PhysicalKey::Code(winit::KeyCode::AltLeft) => Some(Key::LeftAlt),
            winit::PhysicalKey::Code(winit::KeyCode::AltRight) => Some(Key::RightAlt),
            winit::PhysicalKey::Code(winit::KeyCode::Space) => Some(Key::Space),
            winit::PhysicalKey::Code(winit::KeyCode::KeyH) => Some(Key::H),
            winit::PhysicalKey::Code(winit::KeyCode::KeyJ) => Some(Key::J),
            winit::PhysicalKey::Code(winit::KeyCode::KeyK) => Some(Key::K),
            winit::PhysicalKey::Code(winit::KeyCode::KeyL) => Some(Key::L),
            _ => None,
        }
    }

    #[inline]
    fn button(button: winit::MouseButton) -> Option<Button> {
        match button {
            winit::MouseButton::Left => Some(Button::Left),
            winit::MouseButton::Middle => Some(Button::Middle),
            winit::MouseButton::Right => Some(Button::Right),
            _ => None,
        }
    }
}
