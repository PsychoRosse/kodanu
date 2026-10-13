use crate::WorldCell;

pub trait System: 'static {
    fn run(&mut self, cell: WorldCell);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    use std::{cell::RefCell, rc::Rc};

    struct Counter(Rc<RefCell<u32>>);

    impl System for Counter {
        fn run(&mut self, _: WorldCell) {
            *self.0.borrow_mut() += 1;
        }
    }

    #[test]
    fn manual_system_impl_runs() {
        let count = Rc::new(RefCell::new(0));
        let mut system = Counter(count.clone());
        let mut world = World::default();

        system.run(WorldCell::from(&mut world));
        system.run(WorldCell::from(&mut world));

        assert_eq!(*count.borrow(), 2);
    }

    #[test]
    fn function_system_dispatches_closure() {
        let count = Rc::new(RefCell::new(0));
        let count_in_system = count.clone();
        let mut system = FunctionSystem::<_, ()>::new(move || {
            *count_in_system.borrow_mut() += 1;
        });
        let mut world = World::default();

        system.run(WorldCell::from(&mut world));

        assert_eq!(*count.borrow(), 1);
    }
}
