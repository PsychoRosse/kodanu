use std::{collections::HashSet, hash::Hash};

pub(crate) struct DeviceState<D: Eq + Hash + Copy> {
    pressed: HashSet<D>,
    just_pressed: HashSet<D>,
    just_released: HashSet<D>,
}

impl<D: Eq + Hash + Copy> Default for DeviceState<D> {
    fn default() -> Self {
        Self {
            pressed: HashSet::default(),
            just_pressed: HashSet::default(),
            just_released: HashSet::default(),
        }
    }
}

impl<D: Eq + Hash + Copy> DeviceState<D> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pressed: HashSet::with_capacity(capacity),
            just_pressed: HashSet::with_capacity(capacity),
            just_released: HashSet::with_capacity(capacity),
        }
    }
}

impl<D: Eq + Hash + Copy> DeviceState<D> {
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
        }
    }

    #[inline]
    pub fn release(&mut self, button: D) {
        if self.pressed.remove(&button) {
            self.just_released.insert(button);
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
}
