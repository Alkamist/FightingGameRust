use crate::analog_stick::AnalogStick;
use crate::button::Button;

#[derive(Copy, Clone)]
pub struct ControllerState {
    pub left_stick: AnalogStick,
    pub c_stick: AnalogStick,
    pub a_button: Button,
    pub b_button: Button,
    pub x_button: Button,
    pub y_button: Button,
    pub z_button: Button,
    pub r_button: Button,
    pub l_button: Button,
    pub start_button: Button,
    pub d_left_button: Button,
    pub d_right_button: Button,
    pub d_down_button: Button,
    pub d_up_button: Button,
}

impl ControllerState {
    pub fn new(dead_zone: f32) -> ControllerState {
        ControllerState{
            left_stick: AnalogStick::new(dead_zone),
            c_stick: AnalogStick::new(dead_zone),
            a_button: Button::new(),
            b_button: Button::new(),
            x_button: Button::new(),
            y_button: Button::new(),
            z_button: Button::new(),
            r_button: Button::new(),
            l_button: Button::new(),
            start_button: Button::new(),
            d_left_button: Button::new(),
            d_right_button: Button::new(),
            d_down_button: Button::new(),
            d_up_button: Button::new()
        }
    }

    pub fn update(&mut self) {
        self.left_stick.update();
        self.c_stick.update();
        self.a_button.update();
        self.b_button.update();
        self.x_button.update();
        self.y_button.update();
        self.z_button.update();
        self.r_button.update();
        self.l_button.update();
        self.start_button.update();
        self.d_left_button.update();
        self.d_right_button.update();
        self.d_down_button.update();
        self.d_up_button.update();
    }
}
