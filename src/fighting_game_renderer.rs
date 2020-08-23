use std::time::{Instant, Duration};

use ggez::timer;
use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, Rect, DrawMode, Mesh};

use crate::fighting_game::FightingGame;
use crate::debug_text::DebugText;
use crate::interpolated_position::InterpolatedPosition;

pub struct FightingGameRenderer {
    game_delta: Duration,
    camera_zoom: f32,
    character_position: InterpolatedPosition,
    debug_text: DebugText,
    time_accumulator: Duration,
    game_update_instant: Instant,
}

impl FightingGameRenderer {
    pub fn new(game_fps: u32) -> FightingGameRenderer {
        FightingGameRenderer {
            game_delta: Duration::from_secs_f32(1.0 / (game_fps as f32)),
            camera_zoom: 6.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
            debug_text: DebugText::new(),
            time_accumulator: Duration::new(0, 0),
            game_update_instant: Instant::now(),
        }
    }

    pub fn on_game_update(&mut self, fighting_game: &FightingGame) {
        self.game_update_instant = Instant::now();
        self.character_position.set(
            fighting_game.player.x(),
            fighting_game.player.y(),
        )
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        let delta = timer::delta(context);
        self.time_accumulator += delta;

        let interpolation = (Instant::now() - self.game_update_instant).as_secs_f32() / self.game_delta.as_secs_f32();

        //let interpolation = self.time_accumulator.as_secs_f32() % self.game_delta.as_secs_f32();
        //let interpolation = 1.0;

        //println!("{}", interpolation);

        let screen_coordinates = graphics::screen_coordinates(context);
        let screen_width = screen_coordinates.w;
        let screen_height = screen_coordinates.h;
        let camera_pixel_x = 0.5 * screen_width;
        let camera_pixel_y = 0.5 * screen_height + 100.0;
        let character_pixel_width = 50.0;
        let character_pixel_height = 100.0;
        let character_pixel_x = self.character_position.x(interpolation) * self.camera_zoom + camera_pixel_x - 0.5 * character_pixel_width;
        let character_pixel_y = -self.character_position.y(interpolation) * self.camera_zoom + camera_pixel_y - character_pixel_height;

        let character = Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            Rect::new(character_pixel_x, character_pixel_y, character_pixel_width, character_pixel_height),
            graphics::WHITE,
        )?;
        let ground = Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            Rect::new(0.0, camera_pixel_y, screen_width, 2000.0),
            graphics::Color::new(0.3, 0.3, 0.3, 1.0),
        )?;

        graphics::draw(context, &character, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ground, (na::Point2::new(0.0, 0.0),))?;

        //self.debug_text.draw(context,
        //                     fighting_game.player.state_as_string(),
        //                     format!("{}", fighting_game.player.state_frame()),
        //                     format!("{:.5}", fighting_game.player.x_velocity()),
        //                     format!("{:.5}", fighting_game.player.y_velocity()),
        //                     format!("{:.4}", fighting_game.input.left_stick.x_axis.value()),
        //                     format!("{:.4}", fighting_game.input.left_stick.y_axis.value()),
        //                     0.5 * screen_width + 20.0,
        //                     0.5 * screen_height + 200.0)?;

        Ok(())
    }
}
