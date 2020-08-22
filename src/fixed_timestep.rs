use std::time::Duration;

pub struct FixedTimestep {
    progress_fraction: f64,
    fixed_delta: Duration,
    accumulator: Duration
}

impl FixedTimestep {
    pub fn new(fixed_fps: f64) -> FixedTimestep {
        FixedTimestep{
            progress_fraction: 0.0,
            fixed_delta: Duration::from_secs_f64(1.0 / fixed_fps),
            accumulator: Duration::new(0, 0)
        }
    }

    pub fn progress_fraction(&self) -> f64 { self.progress_fraction }

    pub fn update<F>(&mut self, delta: Duration, mut update_fn: F)
        where F: FnMut()
    {
        self.accumulator += delta;
        while self.accumulator >= self.fixed_delta {
            update_fn();
            self.accumulator -= self.fixed_delta;
        }
        self.progress_fraction = self.accumulator.as_secs_f64() / self.fixed_delta.as_secs_f64();
    }
}