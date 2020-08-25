use crate::analog_axis::AnalogAxis;

#[derive(Copy, Clone)]
pub struct AnalogStick {
    pub x_axis: AnalogAxis,
    pub y_axis: AnalogAxis
}

impl AnalogStick {
    pub fn new(dead_zone: f64) -> AnalogStick {
        AnalogStick{
            x_axis: AnalogAxis::new(dead_zone),
            y_axis: AnalogAxis::new(dead_zone)
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x_axis.value().powi(2) + self.y_axis.value().powi(2)).sqrt()
    }

    pub fn set_magnitude(&mut self, value: f64) {
        let current_magnitude = self.magnitude();
        if current_magnitude != 0.0 {
            let scale_factor = value / current_magnitude;
            self.x_axis.set_value(self.x_axis.value() * scale_factor);
            self.y_axis.set_value(self.y_axis.value() * scale_factor);
        }
    }

    pub fn angle(&self) -> f64 {
        let x = self.x_axis.value();
        let y = self.y_axis.value();
        if x != 0.0 || y != 0.0 {
            return y.atan2(x)
        }
        0.0
    }

    pub fn convert_to_melee_values(&mut self) {
        if self.magnitude() > 1.0 {
            fn convert_axis(axis: &mut AnalogAxis) {
                let axis_magnitude = axis.magnitude();
                let axis_round = (axis.value() * 80.0).round() / 80.0;
                if axis_round.abs() > axis_magnitude {
                    axis.set_value(axis.direction() * (axis_magnitude * 80.0).floor() / 80.0);
                }
                else {
                    axis.set_value(axis_round);
                }
            }
            self.set_magnitude(1.0);
            convert_axis(&mut self.x_axis);
            convert_axis(&mut self.y_axis);
        }
        else {
            self.x_axis.set_value((self.x_axis.value() * 80.0).round() / 80.0);
            self.y_axis.set_value((self.y_axis.value() * 80.0).round() / 80.0);
        }
    }

    pub fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
    }
}
