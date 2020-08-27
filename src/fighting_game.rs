use crate::controller_state::ControllerState;
use crate::fighter::Fighter;
use crate::fighting_stage::FightingStage;
use crate::game_math::*;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter,
    pub stage: FightingStage,
    is_paused: bool,
}

impl FightingGame {
    pub fn new() -> FightingGame {
        FightingGame{
            input: ControllerState::new(0.2875),
            player: Fighter::fox(),
            is_paused: false,
            stage: FightingStage::new(),
        }
    }

    pub fn update(&mut self, input: &ControllerState) {
        self.input.copy_inputs(input);
        self.input.convert_to_melee_values();

        let mut frame_advance = false;

        if self.input.start_button.just_pressed() {
            self.is_paused = !self.is_paused;
        }
        if self.is_paused && self.input.z_button.just_pressed() {
            frame_advance = true;
        }

        if !self.is_paused || frame_advance {
            self.player.update(&self.input);
            self.resolve_ground_collisions();
        }

        self.input.update();
    }

    fn resolve_ground_collisions(&mut self) {
        for ground in self.stage.grounds() {
            let ground_length = ground.len();
            if ground_length > 1 {
                for i in 1..ground_length {
                    let i_previous = i - 1;
                    let ground_line_line = LineSegment2D::new(
                        Point2D::new(ground[i_previous][0], ground[i_previous][1]),
                        Point2D::new(ground[i][0], ground[i][1]),
                    );
                    let position_previous = Point2D::new(self.player.x_previous(), self.player.y_previous());
                    let position = Point2D::new(self.player.x(), self.player.y());
                    if let Some(collision_position) = self.player.ecb().get_ground_line_collision_position(position_previous, position, ground_line_line) {
                        if self.player.l_button().is_pressed() {
                            println!("{}, {}", collision_position.x(), collision_position.y());
                            self.player.set_x(collision_position.x());
                            self.player.set_y(collision_position.y());
                        }
                    }
                }
            }
        }
    }
}
