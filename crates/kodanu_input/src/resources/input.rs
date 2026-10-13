use crate::{ActionMap, Axis, DeviceState};

use {
    kodanu_ecs::{EventReader, ResMut, Resource},
    kodanu_math::{MouseDelta, MousePos, MouseScroll, MouseScrollDelta},
};

pub use kodanu_window::{
    Button, ButtonStatus, Key, KeyStatus, KeyboardEvent, MouseEvent, MouseWheelEvent,
};

#[derive(Resource, Default)]
pub struct Input {
    action_map: ActionMap,
    keyboard: DeviceState<Key>,
    mouse: DeviceState<Button>,
    mouse_delta: MouseDelta,
    mouse_wheel_delta: MouseScrollDelta,
    mouse_position: Option<MousePos>,
}

impl Input {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            action_map: ActionMap::with_capacity(capacity),
            keyboard: DeviceState::with_capacity(capacity),
            mouse: DeviceState::with_capacity(capacity),
            mouse_delta: MouseDelta::default(),
            mouse_wheel_delta: MouseScrollDelta::default(),
            mouse_position: None,
        }
    }
}

impl Input {
    pub(crate) fn pre_input_system(
        mut input: ResMut<Input>,
        keyboard: EventReader<KeyboardEvent>,
        mouse: EventReader<MouseEvent>,
        position: EventReader<MousePos>,
        wheel: EventReader<MouseWheelEvent>,
    ) {
        for event in keyboard.iter() {
            let key = event.key();

            match event.state() {
                KeyStatus::Pressed => {
                    input.keyboard.press(key);
                }
                KeyStatus::Released => {
                    input.keyboard.release(key);
                }
            }
        }

        for event in mouse.iter() {
            let button = event.button();

            match event.state() {
                ButtonStatus::Pressed => {
                    input.mouse.press(button);
                }
                ButtonStatus::Released => {
                    input.mouse.release(button);
                }
            }
        }

        for event in position.iter() {
            if let Some(pervious) = input.mouse_position {
                input.mouse_delta += *event - pervious;
            }

            input.mouse_position = Some(*event);
        }

        for event in wheel.iter() {
            match event.delta() {
                MouseScroll::Lines(lines) => {
                    input.mouse_wheel_delta = MouseScrollDelta::new(
                        lines.x() * MouseScrollDelta::DEFAULT_MULTIPLE,
                        lines.y() * MouseScrollDelta::DEFAULT_MULTIPLE,
                    )
                }
                MouseScroll::Pixels(pixels) => {
                    input.mouse_wheel_delta = MouseScrollDelta::new(pixels.x(), pixels.y())
                }
            }
        }
    }

    pub(crate) fn post_input_system(mut input: ResMut<Input>) {
        input.keyboard.clear();
        input.mouse.clear();

        input.mouse_delta = MouseDelta::default();
        input.mouse_wheel_delta = MouseScrollDelta::default();
    }
}

impl Input {
    pub fn axis(&self, axis: Axis) -> f32 {
        self.action_map.axis(axis, self)
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    pub fn key_just_released(&self, key: Key) -> bool {
        self.keyboard.is_just_released(key)
    }

    pub fn button_pressed(&self, button: Button) -> bool {
        self.mouse.is_pressed(button)
    }

    pub fn button_just_pressed(&self, button: Button) -> bool {
        self.mouse.is_just_pressed(button)
    }

    pub fn button_just_released(&self, button: Button) -> bool {
        self.mouse.is_just_released(button)
    }
}

impl Input {
    pub fn mouse_position(&self) -> Option<MousePos> {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> MouseDelta {
        self.mouse_delta
    }

    pub fn wheel_delta(&self) -> MouseScrollDelta {
        self.mouse_wheel_delta
    }
}
