use std::time::Duration;

use piston_window::*;

use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;
use crate::interpolated_position::InterpolatedPosition;
use crate::controller_state::ControllerState;
use crate::fighter::Fighter;

pub struct RenderedFightingGame {
    fighting_game: FightingGame,
    fixed_timestep: FixedTimestep,
    camera_zoom: f64,
    character_position: InterpolatedPosition,
    glyphs: Glyphs,
}

impl RenderedFightingGame {
    pub fn new(window: &mut PistonWindow) -> RenderedFightingGame {
        const GAME_FPS: f64 = 60.0;
        RenderedFightingGame {
            fighting_game: FightingGame::new(),
            fixed_timestep: FixedTimestep::new(GAME_FPS),
            camera_zoom: 6.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
            glyphs: window.load_font("C:/Windows/Fonts/consola.ttf").unwrap(),
        }
    }

    pub fn input(&self) -> &ControllerState { &self.fighting_game.input }
    pub fn player(&self) -> &Fighter { &self.fighting_game.player }

    pub fn update(&mut self, delta: Duration, input: &ControllerState) {
        let mut game_updated = false;
        let fighting_game = &mut self.fighting_game;
        self.fixed_timestep.update(delta, || {
            fighting_game.update(input);
            game_updated = true;
        });

        if game_updated {
            self.on_game_update();
        }
    }

    fn on_game_update(&mut self) {
        self.character_position.set(
            self.fighting_game.player.x() as f64,
            self.fighting_game.player.y() as f64,
        )
    }

    fn to_screen_x(&self, value: f64, window_width: f64) -> f64 {
        value * self.camera_zoom + 0.5 * window_width
    }
    fn to_screen_y(&self, value: f64, window_height: f64) -> f64 {
        -value * self.camera_zoom + 0.5 * window_height + 130.0
    }

    pub fn render(&mut self, context: Context, graphics: &mut G2d, device: &mut gfx_device_gl::Device, window_width: f64, window_height: f64) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        self.draw_character(context, graphics, window_width, window_height);
        self.draw_debug_text(context, graphics, device, window_width, window_height);
    }

    fn draw_character(&mut self, context: Context, graphics: &mut G2d, window_width: f64, window_height: f64) {
        let interpolation = self.fixed_timestep.interpolation();

        // Draw the main character body.
        let character_pixel_width = 50.0;
        let character_pixel_height = 100.0;
        let character_pixel_x = self.to_screen_x(self.character_position.x(interpolation), window_width) - 0.5 * character_pixel_width;
        let character_pixel_y = self.to_screen_y(self.character_position.y(interpolation), window_height) - character_pixel_height;
        rectangle(
            [0.5, 0.5, 0.5, 1.0],
            rectangle::rectangle_by_corners(0.0, 0.0, character_pixel_width, character_pixel_height),
            context.transform.trans(character_pixel_x, character_pixel_y),
            graphics,
        );

        // Draw a way to tell which way the character is facing.
        let character_facing_width = 12.0;
        let character_facing_offset = (character_pixel_width - character_facing_width) * (0.5 * (self.player().facing_direction() + 1.0)) as f64;
        rectangle(
            [0.9, 0.9, 0.9, 1.0],
            rectangle::rectangle_by_corners(0.0, 0.0, character_facing_width, character_pixel_height),
            context.transform.trans(character_pixel_x + character_facing_offset, character_pixel_y),
            graphics,
        );
    }

    fn draw_debug_text(&mut self, context: Context, graphics: &mut G2d, device: &mut gfx_device_gl::Device, window_width: f64, window_height: f64) {
        let game = &mut self.fighting_game;
        let debug_text_pixel_x = 0.5 * window_width;
        let debug_text_pixel_y = 0.5 * window_height + 250.0;
        let color = [0.2, 0.9, 0.2, 1.0];
        let x_spacing = 150.0;
        let y_spacing = 25.0;
        let offset = 50.0;

        Text::new_color(color, 20).draw(
            &game.player.state_as_string()[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x, debug_text_pixel_y),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{}", game.player.state_frame())[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.5}", game.player.x_velocity())[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.5}", game.player.y_velocity())[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.4}", game.input.left_stick.x_axis.value())[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - 2.0 * x_spacing, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.4}", game.input.left_stick.y_axis.value())[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - 2.0 * x_spacing, debug_text_pixel_y),
            graphics,
        ).unwrap();

        self.glyphs.factory.encoder.flush(device);
    }

//    pub fn render(&mut self, event: &Event, window: &mut PistonWindow) {
//        let interpolation = self.fixed_timestep.interpolation();
//
//        const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
//        const CHARACTER_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
//        const CHARACTER_FACING_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
//        const GROUND_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
//
//        let window_size = window.size();
//        let window_width = window_size.width;
//        let window_height = window_size.height;
//
//        let character_pixel_width = 50.0;
//        let character_pixel_height = 100.0;
//        let character_pixel_x = self.to_screen_x(self.character_position.x(interpolation), window_width) - 0.5 * character_pixel_width;
//        let character_pixel_y = self.to_screen_y(self.character_position.y(interpolation), window_height) - character_pixel_height;
//
//        //let ground_rect = rectangle::rectangle_by_corners(0.0, 0.0, window_width, 50.0);
//        let character_rect = rectangle::rectangle_by_corners(0.0, 0.0, character_pixel_width, character_pixel_height);
//        let character_facing_width = 12.0;
//        let character_facing_rect = rectangle::rectangle_by_corners(0.0, 0.0, character_facing_width, character_pixel_height);
//        let character_facing_offset = (character_pixel_width - character_facing_width) * (0.5 * (self.player().facing_direction() + 1.0)) as f64;
//
//        let stage_grounds = self.fighting_game.stage.grounds();
//        let stage_grounds_len = stage_grounds.len();
//        let stage_left_walls = self.fighting_game.stage.left_walls();
//        let stage_left_walls_len = stage_left_walls.len();
//        let stage_right_walls = self.fighting_game.stage.right_walls();
//        let stage_right_walls_len = stage_right_walls.len();
//
//
//        window.draw_2d(event, |c, g, _| {
//            clear(BACKGROUND_COLOR, g);
//            //rectangle(GROUND_COLOR, ground_rect, c.transform.trans(0.0, camera_pixel_y), g);
//            rectangle(CHARACTER_COLOR, character_rect, c.transform.trans(character_pixel_x, character_pixel_y), g);
//            rectangle(CHARACTER_FACING_COLOR, character_facing_rect, c.transform.trans(character_pixel_x + character_facing_offset, character_pixel_y), g);
//
//            // Draw stage grounds.
//            if stage_grounds_len > 1 {
//                for i in 1..stage_grounds_len {
//                    let i_previous = i - 1;
//                    line(
//                        GROUND_COLOR,
//                        1.0,
//                        [
//                            self.to_screen_x(stage_grounds[i_previous][0] as f64, window_width),
//                            self.to_screen_y(stage_grounds[i_previous][1] as f64, window_height),
//                            self.to_screen_x(stage_grounds[i][0] as f64, window_width),
//                            self.to_screen_y(stage_grounds[i][1] as f64, window_height),
//                        ],
//                        c.transform.trans(0.0, 0.0),
//                        g,
//                    );
//                }
//            }
//
//            // Draw stage left walls.
//            if stage_left_walls_len > 1 {
//                for i in 1..stage_left_walls_len {
//                    let i_previous = i - 1;
//                    line(
//                        GROUND_COLOR,
//                        1.0,
//                        [
//                            self.to_screen_x(stage_left_walls[i_previous][0] as f64, window_width),
//                            self.to_screen_y(stage_left_walls[i_previous][1] as f64, window_height),
//                            self.to_screen_x(stage_left_walls[i][0] as f64, window_width),
//                            self.to_screen_y(stage_left_walls[i][1] as f64, window_height),
//                        ],
//                        c.transform.trans(0.0, 0.0),
//                        g,
//                    );
//                }
//            }
//
//            // Draw stage right walls.
//            if stage_right_walls_len > 1 {
//                for i in 1..stage_right_walls_len {
//                    let i_previous = i - 1;
//                    line(
//                        GROUND_COLOR,
//                        1.0,
//                        [
//                            self.to_screen_x(stage_right_walls[i_previous][0] as f64, window_width),
//                            self.to_screen_y(stage_right_walls[i_previous][1] as f64, window_height),
//                            self.to_screen_x(stage_right_walls[i][0] as f64, window_width),
//                            self.to_screen_y(stage_right_walls[i][1] as f64, window_height),
//                        ],
//                        c.transform.trans(0.0, 0.0),
//                        g,
//                    );
//                }
//            }
//        });
//
//        self.debug_text.draw(&self.fighting_game, window, event);
//    }
}
