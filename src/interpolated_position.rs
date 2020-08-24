pub struct InterpolatedPosition {
    x_current: f64,
    x_previous: f64,
    y_current: f64,
    y_previous: f64,
}

impl InterpolatedPosition {
    pub fn new(x: f64, y: f64) -> InterpolatedPosition {
        InterpolatedPosition{
            x_current: x,
            x_previous: x,
            y_current: y,
            y_previous: y,
        }
    }

    pub fn x(&self, interpolation: f64) -> f64 {
        interpolation * (self.x_current - self.x_previous) + self.x_previous
    }
    pub fn set_x(&mut self, value: f64) {
        self.x_previous = self.x_current;
        self.x_current = value;
    }

    pub fn y(&self, interpolation: f64) -> f64 {
        interpolation * (self.y_current - self.y_previous) + self.y_previous
    }
    pub fn set_y(&mut self, value: f64) {
        self.y_previous = self.y_current;
        self.y_current = value;
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
}
