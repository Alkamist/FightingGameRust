pub struct Button {
    pub is_pressed: bool,
    pub was_previously_pressed: bool,
}

impl Button {
    pub fn default() -> Button {
        Button {
            is_pressed: false,
            was_previously_pressed: false,
        }
    }

    pub fn just_pressed(&self) -> bool {
        self.is_pressed && !self.was_previously_pressed
    }

    pub fn just_released(&self) -> bool {
        self.was_previously_pressed && !self.is_pressed
    }

    pub fn update(&mut self) {
        self.was_previously_pressed = self.is_pressed;
    }
}
