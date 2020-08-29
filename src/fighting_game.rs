use crate::point_math::Point;
use crate::line_math::PolyLine;
use crate::controller_state::ControllerState;
use crate::fighter::Fighter;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter,
    pub is_paused: bool,
    pub collision_poly_lines: Vec<PolyLine>,
}

impl FightingGame {
    pub fn default() -> FightingGame {
        FightingGame{
            input: ControllerState::default(),
            player: Fighter::fox(),
            is_paused: false,
            collision_poly_lines: vec![
                PolyLine::from_points(&vec![
                    Point { x: -54.0, y: -100.0 },
                    Point { x: -54.0, y: -47.0 },
                    Point { x: -53.0, y: -46.0 },
                    Point { x: -53.0, y: -31.0 },
                    Point { x: -54.0, y: -30.0 },
                    Point { x: -54.0, y: -28.0 },
                    Point { x: -53.0, y: -27.0 },
                    Point { x: -53.0, y: -12.0 },
                    Point { x: -54.0, y: -11.0 },
                    Point { x: -55.0, y: -8.0 },
                    Point { x: -56.0, y: -7.0 },

                    Point { x: -56.0, y: -3.5 },
                    Point { x: -39.0, y: 0.0 },
                    Point { x: 39.0, y: 0.0 },
                    Point { x: 56.0, y: -3.5 },

                    Point { x: 56.0, y: -7.0 },
                    Point { x: 55.0, y: -8.0 },
                    Point { x: 54.0, y: -11.0 },
                    Point { x: 53.0, y: -12.0 },
                    Point { x: 53.0, y: -27.0 },
                    Point { x: 54.0, y: -28.0 },
                    Point { x: 54.0, y: -30.0 },
                    Point { x: 53.0, y: -31.0 },
                    Point { x: 53.0, y: -46.0 },
                    Point { x: 54.0, y: -47.0 },
                    Point { x: 54.0, y: -100.0 },

                    Point { x: -54.0, y: -100.0 },
                ])
            ],
        }
    }

    pub fn update(&mut self, input: &ControllerState) {
        self.input.update();
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
        }
    }
}

//    fn resolve_collisions(&mut self) {
//        for poly_line in &self.collision_poly_lines {
//            let collision_lines = poly_line.collision_lines();
//            for collision_line in collision_lines {
//                let collision_line_segment = collision_line.line();
//
//                // Ground collision.
//
//                let possible_collision = self.player.ecb().get_ground_line_collision_position(
//                    self.player.position_previous(),
//                    self.player.position(),
//                    collision_line_segment,
//                );
//                if let Some(collision_position) = possible_collision {
//                    //if self.player.can_land() && self.player.velocity().dot(collision_line_segment.normal()) <= 0.0 {
//                    if self.player.velocity().dot(collision_line_segment.normal()) <= 0.0 {
//                        self.player.set_x(collision_position.x());
//                        self.player.set_y(collision_position.y());
//                        //self.player.set_ground_line(Some(ground_line));
//                        //self.player.land();
//                    }
//                }
//            }
//        }
//    }
//}
