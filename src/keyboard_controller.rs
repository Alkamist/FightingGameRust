use piston_window::*;

use crate::controller_state::ControllerState;

pub struct KeyboardController {
    pub controller_state: ControllerState,
    left_state: bool,
    right_state: bool,
    down_state: bool,
    up_state: bool,
    x_state: bool,
    y_state: bool,
    z_state: bool,
    l_state: bool,
    r_state: bool,
    start_state: bool,
}

impl KeyboardController {
    pub fn new(dead_zone: f64) -> KeyboardController {
        KeyboardController {
            controller_state: ControllerState::new(dead_zone),
            left_state: false,
            right_state: false,
            down_state: false,
            up_state: false,
            x_state: false,
            y_state: false,
            z_state: false,
            l_state: false,
            r_state: false,
            start_state: false,
        }
    }

    pub fn update(&mut self, event: &Event) {
        if let Some(args) = event.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => self.left_state = true,
                        Key::D => self.right_state = true,
                        Key::S => self.down_state = true,
                        Key::W => self.up_state = true,
                        Key::Backslash => self.x_state = true,
                        Key::LeftBracket => self.y_state = true,
                        Key::Equals => self.z_state = true,
                        Key::Semicolon => self.l_state = true,
                        Key::RightBracket => self.r_state = true,
                        Key::D5 => self.start_state = true,
                        _ => ()
                    },
                    _ => ()
                },
                ButtonState::Release => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => self.left_state = false,
                        Key::D => self.right_state = false,
                        Key::S => self.down_state = false,
                        Key::W => self.up_state = false,
                        Key::Backslash => self.x_state = false,
                        Key::LeftBracket => self.y_state = false,
                        Key::Equals => self.z_state = false,
                        Key::Semicolon => self.l_state = false,
                        Key::RightBracket => self.r_state = false,
                        Key::D5 => self.start_state = false,
                        _ => ()
                    },
                    _ => ()
                }
            }

            self.controller_state.left_stick.x_axis.set_value_from_states(self.left_state, self.right_state);
            self.controller_state.left_stick.y_axis.set_value_from_states(self.down_state, self.up_state);
            self.controller_state.x_button.set_pressed(self.x_state);
            self.controller_state.y_button.set_pressed(self.y_state);
            self.controller_state.z_button.set_pressed(self.z_state);
            self.controller_state.l_button.set_pressed(self.l_state);
            self.controller_state.r_button.set_pressed(self.r_state);
            self.controller_state.start_button.set_pressed(self.start_state);
        }
    }
}
