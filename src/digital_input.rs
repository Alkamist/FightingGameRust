use piston_window::*;

use crate::controller_state::ControllerState;

pub struct DigitalInput {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub up: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub l: bool,
    pub r: bool,
    pub start: bool,
    pub d_left: bool,
    pub d_right: bool,
    pub d_down: bool,
    pub d_up: bool,
}

impl DigitalInput {
    pub fn default() -> Self {
        Self {
            left: false,
            right: false,
            down: false,
            up: false,
            x: false,
            y: false,
            z: false,
            l: false,
            r: false,
            start: false,
            d_left: false,
            d_right: false,
            d_down: false,
            d_up: false,
        }
    }

    pub fn update_controller_state(&self, controller_state: &mut ControllerState) {
        controller_state.x_axis.set_value_from_states(self.left, self.right);
        controller_state.y_axis.set_value_from_states(self.down, self.up);
        controller_state.x_button.is_pressed = self.x;
        controller_state.y_button.is_pressed = self.y;
        controller_state.z_button.is_pressed = self.z;
        controller_state.l_button.is_pressed = self.l;
        controller_state.r_button.is_pressed = self.r;
        controller_state.start_button.is_pressed = self.start;
        controller_state.d_left_button.is_pressed = self.d_left;
        controller_state.d_right_button.is_pressed = self.d_right;
        controller_state.d_down_button.is_pressed = self.d_down;
        controller_state.d_up_button.is_pressed = self.d_up;
        controller_state.convert_to_melee_values();
    }

    pub fn update_states_with_piston_window_event(&mut self, event: &Event) {
        if let Some(args) = event.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => self.left = true,
                        Key::D => self.right = true,
                        Key::S => self.down = true,
                        Key::W => self.up = true,
                        Key::Backslash => self.x = true,
                        Key::LeftBracket => self.y = true,
                        Key::Equals => self.z = true,
                        Key::Semicolon => self.l = true,
                        Key::RightBracket => self.r = true,
                        Key::D5 => self.start = true,
                        Key::V => self.d_left = true,
                        Key::N => self.d_right = true,
                        Key::B => self.d_down = true,
                        Key::G => self.d_up = true,
                        _ => ()
                    },
                    _ => ()
                },
                ButtonState::Release => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => self.left = false,
                        Key::D => self.right = false,
                        Key::S => self.down = false,
                        Key::W => self.up = false,
                        Key::Backslash => self.x = false,
                        Key::LeftBracket => self.y = false,
                        Key::Equals => self.z = false,
                        Key::Semicolon => self.l = false,
                        Key::RightBracket => self.r = false,
                        Key::D5 => self.start = false,
                        Key::V => self.d_left = false,
                        Key::N => self.d_right = false,
                        Key::B => self.d_down = false,
                        Key::G => self.d_up = false,
                        _ => ()
                    },
                    _ => ()
                }
            }
        }
    }
}
