use crate::analog_axis::{self, AnalogAxis};
use crate::button::Button;

pub struct ControllerState {
    pub x_axis: AnalogAxis,
    pub y_axis: AnalogAxis,
    pub c_x_axis: AnalogAxis,
    pub c_y_axis: AnalogAxis,
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
    pub fn default() -> Self {
        Self {
            x_axis: AnalogAxis::default(),
            y_axis: AnalogAxis::default(),
            c_x_axis: AnalogAxis::default(),
            c_y_axis: AnalogAxis::default(),
            a_button: Button::default(),
            b_button: Button::default(),
            x_button: Button::default(),
            y_button: Button::default(),
            z_button: Button::default(),
            r_button: Button::default(),
            l_button: Button::default(),
            start_button: Button::default(),
            d_left_button: Button::default(),
            d_right_button: Button::default(),
            d_down_button: Button::default(),
            d_up_button: Button::default(),
        }
    }

    pub fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
        self.c_x_axis.update();
        self.c_y_axis.update();
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

    pub fn copy_inputs(&mut self, from_controller: &Self) {
        self.x_axis.value = from_controller.x_axis.value;
        self.y_axis.value = from_controller.y_axis.value;
        self.c_x_axis.value = from_controller.c_x_axis.value;
        self.c_y_axis.value = from_controller.c_y_axis.value;
        self.a_button.is_pressed = from_controller.a_button.is_pressed;
        self.b_button.is_pressed = from_controller.b_button.is_pressed;
        self.x_button.is_pressed = from_controller.x_button.is_pressed;
        self.y_button.is_pressed = from_controller.y_button.is_pressed;
        self.z_button.is_pressed = from_controller.z_button.is_pressed;
        self.r_button.is_pressed = from_controller.r_button.is_pressed;
        self.l_button.is_pressed = from_controller.l_button.is_pressed;
        self.start_button.is_pressed = from_controller.start_button.is_pressed;
        self.d_left_button.is_pressed = from_controller.d_left_button.is_pressed;
        self.d_right_button.is_pressed = from_controller.d_right_button.is_pressed;
        self.d_down_button.is_pressed = from_controller.d_down_button.is_pressed;
        self.d_up_button.is_pressed = from_controller.d_up_button.is_pressed;
    }

    pub fn convert_to_melee_values(&mut self) {
        analog_axis::convert_to_melee_values(&mut self.x_axis, &mut self.y_axis);
        analog_axis::convert_to_melee_values(&mut self.c_x_axis, &mut self.c_y_axis);
    }
}
