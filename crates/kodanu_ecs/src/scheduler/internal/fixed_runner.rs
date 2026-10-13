pub(crate) struct FixedRunner {
    time_step: f32,
    accumulator: f32,
    max_steps: usize,
}

impl Default for FixedRunner {
    fn default() -> Self {
        Self::new(1.0 / 60.0)
    }
}

impl FixedRunner {
    pub fn new(time_step: f32) -> Self {
        Self {
            time_step,
            accumulator: 0.0,
            max_steps: 4,
        }
    }
}

impl FixedRunner {
    pub fn consume(&mut self, delta: f32) -> usize {
        self.accumulator += delta;

        let mut steps = 0;

        while self.accumulator >= self.time_step && steps < self.max_steps {
            self.accumulator -= self.time_step;
            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn does_not_run_before_time_step() {
        let mut runner = FixedRunner::new(0.1);

        assert_eq!(runner.consume(0.05), 0)
    }

    #[test]
    fn runs_one_step_when_enough_time_accumulates() {
        let mut runner = FixedRunner::new(0.1);

        assert_eq!(runner.consume(0.1), 1);
    }

    #[test]
    fn accumulates_time_across_calls() {
        let mut runner = FixedRunner::new(0.1);

        assert_eq!(runner.consume(0.06), 0);
        assert_eq!(runner.consume(0.06), 1);
    }

    #[test]
    fn limits_steps_per_call() {
        let mut runner = FixedRunner::new(0.1);

        assert_eq!(runner.consume(1.0), 4);
    }

    #[test]
    fn preserves_remaining_accumulated_time() {
        let mut runner = FixedRunner::new(0.1);

        assert_eq!(runner.consume(0.55), 4);
        assert_eq!(runner.consume(0.0), 1);
    }

    #[test]
    fn caps_at_max_steps_and_keeps_debt() {
        let mut runner = FixedRunner::default();

        let dt = 1.0 / 60.0;

        assert_eq!(runner.consume(10.0 * dt), 4);
        assert_eq!(runner.consume(0.0), 4);
        assert_eq!(runner.consume(0.0), 2);
    }
}
