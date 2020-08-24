use crate::controller_state::ControllerState;
use crate::fighter::Fighter;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter,
    is_paused: bool,
}

impl FightingGame {
    pub fn new() -> FightingGame {
        FightingGame{
            input: ControllerState::new(0.2875),
            player: Fighter::fox(),
            is_paused: false,
        }
    }

    pub fn update(&mut self, input: &ControllerState) {
        self.input.copy_inputs(input);

        let mut frame_advance = false;

        if self.input.start_button.just_pressed() {
            self.is_paused = !self.is_paused;
        }
        if self.is_paused && self.input.z_button.just_pressed() {
            frame_advance = true;
        }

        if !self.is_paused || frame_advance {
            self.player.update(&self.input);
        }

        self.input.update();
    }
}
