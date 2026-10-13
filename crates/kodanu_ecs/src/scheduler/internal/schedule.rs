use crate::{System, WorldCell};

#[derive(Default)]
pub(crate) struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            systems: Vec::with_capacity(capacity),
        }
    }
}

impl Schedule {
    pub fn add<S: System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self, world: WorldCell) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn empty_schedule_runs_without_panic() {
        let mut schedule = Schedule::default();
        let mut world = World::default();

        schedule.run(WorldCell::from(&mut world));
    }

    #[test]
    fn systems_run_in_insertion_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut schedule = Schedule::default();
        let mut world = World::default();

        for label in ["a", "b", "c"] {
            let log = log.clone();

            schedule.add(FunctionSystem::<_, ()>::new(move || {
                log.borrow_mut().push(label);
            }));
        }

        schedule.run(WorldCell::from(&mut world));

        assert_eq!(*log.borrow(), ["a", "b", "c"]);
    }

    #[test]
    fn schedule_runs_multiple_times() {
        let count = Rc::new(RefCell::new(0));
        let mut schedule = Schedule::default();
        let mut world = World::default();
        let count_in_system = count.clone();
        
        schedule.add(FunctionSystem::<_, ()>::new(move || {
            *count_in_system.borrow_mut() += 1;
        }));

        let cell = WorldCell::from(&mut world);

        schedule.run(cell);
        schedule.run(cell);

        assert_eq!(*count.borrow(), 2);
    }
}
