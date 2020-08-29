use crate::vector_math::Vector;

pub struct AnalogAxis {
    pub value: f64,
    pub previous_value: f64,
    pub dead_zone: f64,
    pub was_previously_active: bool,
    pub frames_active: u32,
    pub high_state_was_first: bool,
}

impl AnalogAxis {
    pub fn default() -> AnalogAxis {
        AnalogAxis {
            value: 0.0,
            previous_value: 0.0,
            dead_zone: 0.2875,
            was_previously_active: false,
            frames_active: 0,
            high_state_was_first: true,
        }
    }

    pub fn direction(&self) -> f64 {
        if self.value > 0.0 {
            return 1.0;
        }
        else if self.value < 0.0 {
            return -1.0;
        }
        0.0
    }
    pub fn just_crossed_center(&self) -> bool {
        (self.value < 0.0 && self.previous_value >= 0.0)
        || (self.value > 0.0 && self.previous_value <= 0.0)
    }
    pub fn is_active(&self) -> bool { self.value.abs() >= self.dead_zone }
    pub fn just_activated(&self) -> bool { self.just_crossed_center() || self.is_active() && !self.was_previously_active }
    pub fn just_deactivated(&self) -> bool { self.was_previously_active && !self.is_active() }

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
            self.value = -1.0;
        }
        else if only_high || (low_and_high && !self.high_state_was_first) {
            self.value = 1.0;
        }
        else {
            self.value = 0.0;
        }
    }

    pub fn update(&mut self) {
        if self.just_activated() {
            self.frames_active = 0;
        }
        else if self.is_active() {
            self.frames_active += 1;
        }
        else {
            self.frames_active = 0;
        }

        self.previous_value = self.value;
        self.was_previously_active = self.is_active();
    }
}

pub fn convert_to_melee_values(x_axis: &mut AnalogAxis, y_axis: &mut AnalogAxis) {
    let mut stick_vector = Vector { x: x_axis.value, y: x_axis.value };
    if stick_vector.magnitude() > 1.0 {
        fn convert_axis(axis: &mut AnalogAxis) {
            let axis_magnitude = axis.value.abs();
            let axis_round = (axis.value * 80.0).round() / 80.0;
            if axis_round.abs() > axis_magnitude {
                axis.value = axis.direction() * (axis_magnitude * 80.0).floor() / 80.0;
            }
            else {
                axis.value = axis_round;
            }
        }
        stick_vector.set_magnitude(1.0);
        x_axis.value = stick_vector.x;
        y_axis.value = stick_vector.y;
        convert_axis(x_axis);
        convert_axis(y_axis);
    }
    else {
        x_axis.value = (x_axis.value * 80.0).round() / 80.0;
        y_axis.value = (y_axis.value * 80.0).round() / 80.0;
    }
}
