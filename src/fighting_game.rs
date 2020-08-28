use crate::controller_state::ControllerState;
use crate::fighter::Fighter;
use crate::game_math::*;
use crate::collision::*;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter,
    collision_poly_lines: Vec<CollisionPolyLine>,
    is_paused: bool,
}

impl FightingGame {
    pub fn new() -> FightingGame {
        FightingGame{
            input: ControllerState::new(0.2875),
            player: Fighter::fox(),
            is_paused: false,
            collision_poly_lines: vec![
                CollisionPolyLine::new(
                    vec![
                        Point2D::new(-54.0, -100.0),
                        Point2D::new(-54.0, -47.0),
                        Point2D::new(-53.0, -46.0),
                        Point2D::new(-53.0, -31.0),
                        Point2D::new(-54.0, -30.0),
                        Point2D::new(-54.0, -28.0),
                        Point2D::new(-53.0, -27.0),
                        Point2D::new(-53.0, -12.0),
                        Point2D::new(-54.0, -11.0),
                        Point2D::new(-55.0, -8.0),
                        Point2D::new(-56.0, -7.0),

                        Point2D::new(-56.0, -3.5),
                        Point2D::new(-39.0, 0.0),
                        Point2D::new(39.0, 0.0),
                        Point2D::new(56.0, -3.5),

                        Point2D::new(56.0, -7.0),
                        Point2D::new(55.0, -8.0),
                        Point2D::new(54.0, -11.0),
                        Point2D::new(53.0, -12.0),
                        Point2D::new(53.0, -27.0),
                        Point2D::new(54.0, -28.0),
                        Point2D::new(54.0, -30.0),
                        Point2D::new(53.0, -31.0),
                        Point2D::new(53.0, -46.0),
                        Point2D::new(54.0, -47.0),
                        Point2D::new(54.0, -100.0),

                        Point2D::new(-54.0, -100.0),
                    ]
                ),
            ],
        }
    }

    pub fn collision_poly_lines(&self) -> &Vec<CollisionPolyLine> { &self.collision_poly_lines }

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
            self.resolve_collisions();
        }

        self.input.update();
    }

    fn resolve_collisions(&mut self) {
        for poly_line in &self.collision_poly_lines {
            let collision_lines = poly_line.collision_lines();
            for collision_line in collision_lines {
                let collision_line_segment = collision_line.line();
                let possible_collision = self.player.ecb().get_ground_line_collision_position(
                    self.player.position_previous(),
                    self.player.position(),
                    collision_line_segment,
                );
                if let Some(collision_position) = possible_collision {
                    if self.player.can_land() && self.player.velocity().dot(collision_line_segment.bottom_normal()) > 0.0 {
                        self.player.set_x(collision_position.x());
                        self.player.set_y(collision_position.y());
                        //self.player.set_ground_line(Some(ground_line));
                        self.player.land();
                    }
                }
            }
        }
    }
}
