use crate::{FixedRunner, IntoSystem, Schedule, Stage, WorldCell};

use std::array::from_fn;

pub struct Scheduler {
    schedules: [Schedule; Stage::COUNT],
    fixed_runner: FixedRunner,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            schedules: from_fn(|_| Schedule::default()),
            fixed_runner: FixedRunner::default(),
        }
    }
}

impl Scheduler {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            schedules: from_fn(|_| Schedule::with_capacity(capacity)),
            fixed_runner: FixedRunner::default(),
        }
    }
}

impl Scheduler {
    pub fn add<M, S>(&mut self, stage: Stage, system: S)
    where
        S: IntoSystem<M>,
    {
        self.schedules[stage.as_usize()].add(system.into_system());
    }

    pub fn run_startup(&mut self, cell: WorldCell) {
        self.run(Stage::Startup, cell);
    }

    pub fn run_full(&mut self, cell: WorldCell, delta: f32) {
        let fixed_steps = self.fixed_runner.consume(delta);

        for _ in 0..fixed_steps {
            self.run(Stage::PreFixedUpdate, cell);
            self.run(Stage::FixedUpdate, cell);
            self.run(Stage::PostFixedUpdate, cell);
        }

        self.run(Stage::TimeUpdate, cell);
        self.run(Stage::PreInputUpdate, cell);

        self.run(Stage::PreUpdate, cell);
        self.run(Stage::Update, cell);
        self.run(Stage::LateUpdate, cell);

        self.run(Stage::PreRender, cell);
        self.run(Stage::Render, cell);
        self.run(Stage::PostRender, cell);

        self.run(Stage::PostInputUpdate, cell);
    }
}

impl Scheduler {
    pub(crate) fn run(&mut self, stage: Stage, cell: WorldCell) {
        self.schedules[stage.as_usize()].run(cell);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    use std::{cell::RefCell, rc::Rc};

    type Log = Rc<RefCell<Vec<&'static str>>>;

    fn record(log: &Log, label: &'static str) -> impl FnMut() + 'static {
        let log = log.clone();

        move || log.borrow_mut().push(label)
    }

    #[test]
    fn run_startup_runs_only_startup_systems() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut scheduler = Scheduler::default();
        let mut world = World::default();

        scheduler.add(Stage::Startup, record(&log, "startup"));
        scheduler.add(Stage::Update, record(&log, "update"));

        scheduler.run_startup(WorldCell::from(&mut world));

        assert_eq!(*log.borrow(), ["startup"]);
    }

    #[test]
    fn run_full_runs_non_fixed_stages_once() {
        let count = Rc::new(RefCell::new(0));
        let mut scheduler = Scheduler::default();
        let mut world = World::default();
        let count_in_system = count.clone();

        scheduler.add(Stage::Update, move || {
            *count_in_system.borrow_mut() += 1;
        });

        scheduler.run_full(WorldCell::from(&mut world), 0.0);

        assert_eq!(*count.borrow(), 1);
    }

    #[test]
    fn run_full_runs_fixed_stages_per_accumulator() {
        let count = Rc::new(RefCell::new(0));
        let mut scheduler = Scheduler::default();
        let mut world = World::default();
        let count_in_system = count.clone();

        scheduler.add(Stage::FixedUpdate, move || {
            *count_in_system.borrow_mut() += 1;
        });

        let dt = 1.0 / 60.0;

        scheduler.run_full(WorldCell::from(&mut world), 1.5 * dt);
        assert_eq!(*count.borrow(), 1);

        scheduler.run_full(WorldCell::from(&mut world), 10.0 * dt);
        assert_eq!(*count.borrow(), 5, "second call is capped at max_steps = 4");
    }

    #[test]
    fn run_full_stage_order_and_startup_isolation() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut scheduler = Scheduler::default();
        let mut world = World::default();
        let dt = 1.0 / 60.0;

        for (stage, label) in [
            (Stage::Startup, "Startup"),
            (Stage::PreFixedUpdate, "PreFixedUpdate"),
            (Stage::FixedUpdate, "FixedUpdate"),
            (Stage::PostFixedUpdate, "PostFixedUpdate"),
            (Stage::TimeUpdate, "TimeUpdate"),
            (Stage::PreInputUpdate, "PreInputUpdate"),
            (Stage::PreUpdate, "PreUpdate"),
            (Stage::Update, "Update"),
            (Stage::LateUpdate, "LateUpdate"),
            (Stage::PreRender, "PreRender"),
            (Stage::Render, "Render"),
            (Stage::PostRender, "PostRender"),
            (Stage::PostInputUpdate, "PostInputUpdate"),
        ] {
            scheduler.add(stage, record(&log, label));
        }

        scheduler.run_startup(WorldCell::from(&mut world));

        assert_eq!(*log.borrow(), ["Startup"]);

        log.borrow_mut().clear();

        scheduler.run_full(WorldCell::from(&mut world), 1.5 * dt);

        assert_eq!(
            *log.borrow(),
            [
                "PreFixedUpdate",
                "FixedUpdate",
                "PostFixedUpdate",
                "TimeUpdate",
                "PreInputUpdate",
                "PreUpdate",
                "Update",
                "LateUpdate",
                "PreRender",
                "Render",
                "PostRender",
                "PostInputUpdate",
            ]
        );
    }
}
