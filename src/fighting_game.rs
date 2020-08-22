use crate::controller_state::ControllerState;
use crate::fighter::Fighter;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter
}

impl FightingGame {
    pub fn new() -> FightingGame {
        FightingGame{
            input: ControllerState::new(0.2875),
            player: Fighter::fox()
        }
    }

    pub fn update(&mut self, input: &mut ControllerState) {
        self.update_internal_controller_state(input);

        self.player.update(self.input);

        self.input.update();
    }

    fn update_internal_controller_state(&mut self, input: &mut ControllerState) {
        self.input.left_stick.x_axis.set_value(input.left_stick.x_axis.value());
        self.input.left_stick.y_axis.set_value(input.left_stick.y_axis.value());
        self.input.c_stick.x_axis.set_value(input.c_stick.x_axis.value());
        self.input.c_stick.y_axis.set_value(input.c_stick.y_axis.value());
        self.input.a_button.set_pressed(input.a_button.is_pressed());
        self.input.b_button.set_pressed(input.b_button.is_pressed());
        self.input.x_button.set_pressed(input.x_button.is_pressed());
        self.input.y_button.set_pressed(input.y_button.is_pressed());
        self.input.z_button.set_pressed(input.z_button.is_pressed());
        self.input.r_button.set_pressed(input.r_button.is_pressed());
        self.input.l_button.set_pressed(input.l_button.is_pressed());
        self.input.start_button.set_pressed(input.start_button.is_pressed());
        self.input.d_left_button.set_pressed(input.d_left_button.is_pressed());
        self.input.d_right_button.set_pressed(input.d_right_button.is_pressed());
        self.input.d_down_button.set_pressed(input.d_down_button.is_pressed());
        self.input.d_up_button.set_pressed(input.d_up_button.is_pressed());
    }
}
