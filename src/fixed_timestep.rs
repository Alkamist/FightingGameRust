use std::time::Duration;

pub struct FixedTimestep {
    interpolation: f32,
    fixed_delta: Duration,
    accumulator: Duration
}

impl FixedTimestep {
    pub fn new(fixed_fps: f32) -> FixedTimestep {
        FixedTimestep{
            interpolation: 0.0,
            fixed_delta: Duration::from_secs_f32(1.0 / fixed_fps),
            accumulator: Duration::new(0, 0)
        }
    }

    pub fn interpolation(&self) -> f32 { self.interpolation }

    pub fn update<F>(&mut self, delta: Duration, mut update_fn: F)
        where F: FnMut()
    {
        self.accumulator += delta;
        while self.accumulator >= self.fixed_delta {
            update_fn();
            self.accumulator -= self.fixed_delta;
        }
        self.interpolation = self.accumulator.as_secs_f32() / self.fixed_delta.as_secs_f32();
    }
}
