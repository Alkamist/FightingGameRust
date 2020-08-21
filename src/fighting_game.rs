use crate::controller_state::ControllerState;

pub struct FightingGame {
    pub controller_state: ControllerState,
    pub x: f32,
}

impl FightingGame {
    pub fn new() -> FightingGame {
        FightingGame{
            controller_state: ControllerState::new(0.2875),
            x: 0.0
        }
    }

    pub fn update(&mut self, controller_state: &mut ControllerState) {
        self.controller_state.left_stick.x_axis.set_value(controller_state.left_stick.x_axis.value());
        self.controller_state.left_stick.y_axis.set_value(controller_state.left_stick.y_axis.value());
        self.controller_state.c_stick.x_axis.set_value(controller_state.c_stick.x_axis.value());
        self.controller_state.c_stick.y_axis.set_value(controller_state.c_stick.y_axis.value());
        self.controller_state.a_button.set_pressed(controller_state.a_button.is_pressed());
        self.controller_state.b_button.set_pressed(controller_state.b_button.is_pressed());
        self.controller_state.x_button.set_pressed(controller_state.x_button.is_pressed());
        self.controller_state.y_button.set_pressed(controller_state.y_button.is_pressed());
        self.controller_state.z_button.set_pressed(controller_state.z_button.is_pressed());
        self.controller_state.r_button.set_pressed(controller_state.r_button.is_pressed());
        self.controller_state.l_button.set_pressed(controller_state.l_button.is_pressed());
        self.controller_state.start_button.set_pressed(controller_state.start_button.is_pressed());
        self.controller_state.d_left_button.set_pressed(controller_state.d_left_button.is_pressed());
        self.controller_state.d_right_button.set_pressed(controller_state.d_right_button.is_pressed());
        self.controller_state.d_down_button.set_pressed(controller_state.d_down_button.is_pressed());
        self.controller_state.d_up_button.set_pressed(controller_state.d_up_button.is_pressed());

        let x_axis = &mut self.controller_state.left_stick.x_axis;

        if x_axis.just_activated() {
            self.x += 60.0 * x_axis.value();
        }

        self.controller_state.update();
    }
}
