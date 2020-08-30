use std::time::Duration;

pub struct FixedTimestep {
    pub interpolation: f64,
    pub fixed_delta: Duration,
    pub accumulator: Duration
}

impl FixedTimestep {
    pub fn with_fixed_fps(fixed_fps: f64) -> Self {
        Self{
            interpolation: 0.0,
            fixed_delta: Duration::from_secs_f64(1.0 / fixed_fps),
            accumulator: Duration::new(0, 0)
        }
    }

    pub fn update<F>(&mut self, delta: Duration, mut update_fn: F)
        where F: FnMut()
    {
        self.accumulator += delta;
        while self.accumulator >= self.fixed_delta {
            update_fn();
            self.accumulator -= self.fixed_delta;
        }
        self.interpolation = self.accumulator.as_secs_f64() / self.fixed_delta.as_secs_f64();
    }
}
