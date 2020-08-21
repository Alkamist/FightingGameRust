#[derive(Copy, Clone)]
pub struct Button {
    state_current: bool,
    state_previous: bool
}

impl Button {
    pub fn new() -> Button {
        Button{
            state_current: false,
            state_previous: false
        }
    }

    pub fn set_pressed(&mut self, value: bool) {
        self.state_current = value;
    }

    pub fn is_pressed(&self) -> bool {
        self.state_current
    }

    pub fn just_pressed(&self) -> bool {
        self.state_current && !self.state_previous
    }

    pub fn just_released(&self) -> bool {
        self.state_previous && !self.state_current
    }

    pub fn update(&mut self) {
        self.state_previous = self.state_current;
    }
}
