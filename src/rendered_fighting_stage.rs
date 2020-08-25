use piston_window::*;

pub struct RenderedFightingStage {
}

impl RenderedFightingStage {
    pub fn new() -> RenderedFightingStage {
        RenderedFightingStage {
        }
    }
    pub fn draw(&mut self, game: &FightingGame, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g, device| {
            // Draw stage grounds.
            if stage_grounds_len > 1 {
                for i in 1..stage_grounds_len {
                    let i_previous = i - 1;
                    line(
                        GROUND_COLOR,
                        1.0,
                        [
                            self.to_screen_x(stage_grounds[i_previous][0] as f64, window_width),
                            self.to_screen_y(stage_grounds[i_previous][1] as f64, window_height),
                            self.to_screen_x(stage_grounds[i][0] as f64, window_width),
                            self.to_screen_y(stage_grounds[i][1] as f64, window_height),
                        ],
                        c.transform.trans(0.0, 0.0),
                        g,
                    );
                }
            }

            // Draw stage left walls.
            if stage_left_walls_len > 1 {
                for i in 1..stage_left_walls_len {
                    let i_previous = i - 1;
                    line(
                        GROUND_COLOR,
                        1.0,
                        [
                            self.to_screen_x(stage_left_walls[i_previous][0] as f64, window_width),
                            self.to_screen_y(stage_left_walls[i_previous][1] as f64, window_height),
                            self.to_screen_x(stage_left_walls[i][0] as f64, window_width),
                            self.to_screen_y(stage_left_walls[i][1] as f64, window_height),
                        ],
                        c.transform.trans(0.0, 0.0),
                        g,
                    );
                }
            }

            // Draw stage right walls.
            if stage_right_walls_len > 1 {
                for i in 1..stage_right_walls_len {
                    let i_previous = i - 1;
                    line(
                        GROUND_COLOR,
                        1.0,
                        [
                            self.to_screen_x(stage_right_walls[i_previous][0] as f64, window_width),
                            self.to_screen_y(stage_right_walls[i_previous][1] as f64, window_height),
                            self.to_screen_x(stage_right_walls[i][0] as f64, window_width),
                            self.to_screen_y(stage_right_walls[i][1] as f64, window_height),
                        ],
                        c.transform.trans(0.0, 0.0),
                        g,
                    );
                }
            }
        });
    }
}
