pub struct InterpolatedPosition {
    x_current: f32,
    x_previous: f32,
    y_current: f32,
    y_previous: f32,
}

impl InterpolatedPosition {
    pub fn new(x: f32, y: f32) -> InterpolatedPosition {
        InterpolatedPosition{
            x_current: x,
            x_previous: x,
            y_current: y,
            y_previous: y,
        }
    }

    pub fn x(&self, interpolation: f32) -> f32 {
        interpolation * (self.x_current - self.x_previous) + self.x_previous
    }
    pub fn set_x(&mut self, value: f32) {
        self.x_previous = self.x_current;
        self.x_current = value;
    }

    pub fn y(&self, interpolation: f32) -> f32 {
        interpolation * (self.y_current - self.y_previous) + self.y_previous
    }
    pub fn set_y(&mut self, value: f32) {
        self.y_previous = self.y_current;
        self.y_current = value;
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
}
