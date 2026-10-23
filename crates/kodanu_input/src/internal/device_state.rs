#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct DeviceState<D: Eq + std::hash::Hash + Copy> {
    pressed: std::collections::HashSet<D>,
    just_pressed: std::collections::HashSet<D>,
    just_released: std::collections::HashSet<D>,
    held_for: std::collections::HashMap<D, std::time::Duration>,
}

impl<D: Eq + std::hash::Hash + Copy> Default for DeviceState<D> {
    fn default() -> Self {
        Self {
            pressed: std::collections::HashSet::default(),
            just_pressed: std::collections::HashSet::default(),
            just_released: std::collections::HashSet::default(),
            held_for: std::collections::HashMap::default(),
        }
    }
}

impl<D: Eq + std::hash::Hash + Copy> DeviceState<D> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pressed: std::collections::HashSet::with_capacity(capacity),
            just_pressed: std::collections::HashSet::with_capacity(capacity),
            just_released: std::collections::HashSet::with_capacity(capacity),
            held_for: std::collections::HashMap::with_capacity(capacity),
        }
    }
}

impl<D: Eq + std::hash::Hash + Copy> DeviceState<D> {
    #[inline]
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    #[inline]
    pub fn contains_all(&self, buttons: &[D]) -> bool {
        buttons.iter().all(|button| self.pressed.contains(button))
    }

    #[inline]
    pub fn press(&mut self, button: D) {
        if self.pressed.insert(button) {
            self.just_pressed.insert(button);
            self.held_for.insert(button, std::time::Duration::ZERO);
        }
    }

    #[inline]
    pub fn release(&mut self, button: D) {
        if self.pressed.remove(&button) {
            self.just_released.insert(button);
            self.held_for.remove(&button);
        }
    }

    #[inline]
    pub fn is_pressed(&self, button: D) -> bool {
        self.pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_pressed(&self, button: D) -> bool {
        self.just_pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_released(&self, button: D) -> bool {
        self.just_released.contains(&button)
    }

    #[inline]
    pub fn is_held_for(&self, button: D, duration: std::time::Duration) -> bool {
        self.held_for
            .get(&button)
            .is_some_and(|elapsed| *elapsed >= duration)
    }

    #[inline]
    pub fn update(&mut self, delta: f32) {
        let delta = std::time::Duration::from_secs_f32(delta);

        for elapsed in self.held_for.values_mut() {
            *elapsed += delta;
        }
    }
}
