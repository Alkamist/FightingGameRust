#[derive(Copy, Clone)]
pub struct AnalogAxis {
    value_current: f64,
    value_previous: f64,
    dead_zone: f64,
    active_previous: bool,
    active_frames: u32,
    high_state_was_first: bool
}

impl AnalogAxis {
    pub fn new(dead_zone: f64) -> AnalogAxis {
        AnalogAxis{
            value_current: 0.0,
            value_previous: 0.0,
            dead_zone: dead_zone,
            active_previous: false,
            active_frames: 0,
            high_state_was_first: true
        }
    }

    pub fn value(&self) -> f64 { self.value_current }
    pub fn magnitude(&self) -> f64 { self.value_current.abs() }
    pub fn direction(&self) -> f64 {
        if self.value_current > 0.0 { return 1.0; }
        else if self.value_current < 0.0 { return -1.0; }
        0.0
    }
    pub fn just_crossed_center(&self) -> bool {
        (self.value_current < 0.0 && self.value_previous >= 0.0)
        || (self.value_current > 0.0 && self.value_previous <= 0.0)
    }
    pub fn is_active(&self) -> bool { self.magnitude() >= self.dead_zone }
    pub fn just_activated(&self) -> bool { self.just_crossed_center() || self.is_active() && !self.active_previous }
    pub fn just_deactivated(&self) -> bool { self.active_previous && !self.is_active() }

    pub fn active_frames(&self) -> u32 { self.active_frames }

    pub fn set_value(&mut self, value: f64) { self.value_current = value; }
    pub fn set_value_from_states(&mut self, low: bool, high: bool) {
        if high && !low {
            self.high_state_was_first = true;
        }
        else if low && !high {
            self.high_state_was_first = false;
        }

        let low_and_high = low && high;
        let only_low = low && !high;
        let only_high = high && !low;

        if only_low || (low_and_high && self.high_state_was_first) {
            self.value_current = -1.0;
        }
        else if only_high || (low_and_high && !self.high_state_was_first) {
            self.value_current = 1.0;
        }
        else {
            self.value_current = 0.0;
        }
    }

    pub fn update(&mut self) {
        if self.just_activated() {
            self.active_frames = 0;
        }
        else if self.is_active() {
            self.active_frames += 1;
        }
        else {
            self.active_frames = 0;
        }

        self.value_previous = self.value_current;
        self.active_previous = self.is_active();
    }
}
