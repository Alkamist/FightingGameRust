use crate::point_math::Point;
use crate::line_math::{PolyLine, LineSegment};
use crate::vector_math::Vector;
use crate::controller_state::ControllerState;
use crate::fighter::Fighter;

pub struct FightingGame {
    pub input: ControllerState,
    pub player: Fighter,
    pub is_paused: bool,
    pub collision_poly_lines: Vec<PolyLine>,
}

impl FightingGame {
    pub fn default() -> Self {
        Self{
            input: ControllerState::default(),
            player: Fighter::fox(),
            is_paused: false,
            collision_poly_lines: vec![
                PolyLine::from_points(&vec![
                    Point { x: -56.0, y: -3.5 },
                    Point { x: -28.0, y: 12.5 },
                    Point { x: -15.0, y: -5.5 },
                    Point { x: -4.0, y: 0.0 },

                    Point { x: 4.0, y: 0.0 },
                    Point { x: 15.0, y: -5.5 },
                    Point { x: 28.0, y: 12.5 },
                    Point { x: 56.0, y: -3.5 },
                ])
            ],
//            collision_poly_lines: vec![
//                PolyLine::from_points(&vec![
//                    Point { x: -54.0, y: -100.0 },
//                    Point { x: -54.0, y: -47.0 },
//                    Point { x: -53.0, y: -46.0 },
//                    Point { x: -53.0, y: -31.0 },
//                    Point { x: -54.0, y: -30.0 },
//                    Point { x: -54.0, y: -28.0 },
//                    Point { x: -53.0, y: -27.0 },
//                    Point { x: -53.0, y: -12.0 },
//                    Point { x: -54.0, y: -11.0 },
//                    Point { x: -55.0, y: -8.0 },
//                    Point { x: -56.0, y: -7.0 },
//
//                    Point { x: -56.0, y: -3.5 },
//                    Point { x: -39.0, y: 0.0 },
//                    Point { x: 39.0, y: 0.0 },
//                    Point { x: 56.0, y: -3.5 },
//
//                    Point { x: 56.0, y: -7.0 },
//                    Point { x: 55.0, y: -8.0 },
//                    Point { x: 54.0, y: -11.0 },
//                    Point { x: 53.0, y: -12.0 },
//                    Point { x: 53.0, y: -27.0 },
//                    Point { x: 54.0, y: -28.0 },
//                    Point { x: 54.0, y: -30.0 },
//                    Point { x: 53.0, y: -31.0 },
//                    Point { x: 53.0, y: -46.0 },
//                    Point { x: 54.0, y: -47.0 },
//                    Point { x: 54.0, y: -100.0 },
//
//                    Point { x: -54.0, y: -100.0 },
//                ])
//            ],
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
            self.resolve_collisions();
        }
    }

    fn resolve_collisions(&mut self) {
        for poly_line in &self.collision_poly_lines {
            for line_segment in &poly_line.segments {
                // Ground collision.
                let possible_collision = self.get_ground_line_collision_position(
                    &self.player,
                    line_segment,
                );
                if let Some(collision_position) = possible_collision {
                    //if self.player.can_land() && self.player.velocity.dot(&line_segment.normal()) <= 0.0 {
                    if self.player.velocity.dot(&line_segment.normal()) <= 0.0 {
                        self.player.position.x = collision_position.x;
                        self.player.position.y = collision_position.y;

                        let collision_normal = line_segment.normal();
                        let normal_component = self.player.velocity.dot(&collision_normal);
                        self.player.velocity.x -= normal_component * collision_normal.x;
                        self.player.velocity.y -= normal_component * collision_normal.y;

                        //self.player.ground_angle = line_segment.direction().angle();
                        //self.player.land()
                    }
                }
            }
        }
    }

    fn get_ground_line_collision_position(
        &self,
        player: &Fighter,
        ground_line: &LineSegment,
    ) -> Option<Point> {

        let tolerance = 0.01;

        let ecb_bottom_x_previous = player.ecb.bottom.x + player.previous_position.x;
        let ecb_bottom_y_previous = player.ecb.bottom.y + player.previous_position.y;
        let ecb_bottom_x = player.ecb.bottom.x + player.position.x;
        let ecb_bottom_y = player.ecb.bottom.y + player.position.y;

        let movement_vector = Vector {
            x: ecb_bottom_x - ecb_bottom_x_previous,
            y: ecb_bottom_y - ecb_bottom_y_previous,
        };
        let movement_vector_direction = movement_vector.direction();

        let movement_line = LineSegment {
            point_a: Point {
                x: ecb_bottom_x_previous,
                y: ecb_bottom_y_previous,
            },
            point_b: Point {
                x: ecb_bottom_x,
                y: ecb_bottom_y,
            },
        };

        // Extend the movement line backward by some tolerance to prevent,
        // moving through obstacles.
        let movement_line_with_tolerance = LineSegment {
            point_a: Point {
                x: ecb_bottom_x_previous - movement_vector_direction.x * tolerance,
                y: ecb_bottom_y_previous - movement_vector_direction.y * tolerance,
            },
            point_b: Point {
                x: ecb_bottom_x,
                y: ecb_bottom_y,
            },
        };

        if movement_line_with_tolerance.intersects_with(ground_line) {
            if let Some(intersection_point) = movement_line_with_tolerance.intersection_with(ground_line) {
                let penetration_vector = Vector {
                    x: movement_line.point_b.x - intersection_point.x,
                    y: movement_line.point_b.y - intersection_point.y,
                };
                let ground_line_direction = ground_line.direction();
                let projection_length = penetration_vector.dot(&ground_line_direction);
                let collision_delta_x = ground_line_direction.x * projection_length;
                let collision_delta_y = ground_line_direction.y * projection_length;
                let projected_point = Point {
                    x: intersection_point.x + collision_delta_x,
                    y: intersection_point.y + collision_delta_y,
                };
                Some(projected_point)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}
