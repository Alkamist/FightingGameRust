use ggez::timer;
use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, Rect, DrawMode, Mesh};

use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;
use crate::debug_text::DebugText;
use crate::interpolated_position::InterpolatedPosition;
use crate::controller_state::ControllerState;

pub struct RenderedFightingGame {
    fighting_game: FightingGame,
    fixed_timestep: FixedTimestep,
    camera_zoom: f32,
    character_position: InterpolatedPosition,
    debug_text: DebugText,
    should_draw_debug_text: bool,
}

impl RenderedFightingGame {
    pub fn new() -> RenderedFightingGame {
        const GAME_FPS: f32 = 60.0;
        RenderedFightingGame {
            fighting_game: FightingGame::new(),
            fixed_timestep: FixedTimestep::new(GAME_FPS),
            camera_zoom: 6.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
            debug_text: DebugText::new(),
            should_draw_debug_text: true,
        }
    }

    pub fn update(&mut self, context: &mut Context, input: &ControllerState) {
        let mut game_updated = false;
        let fighting_game = &mut self.fighting_game;
        self.fixed_timestep.update(timer::delta(context), || {
            fighting_game.update(input);
            game_updated = true;
        });

        if game_updated {
            self.on_game_update();
        }
    }

    fn on_game_update(&mut self) {
        self.update_debug_text();
        self.character_position.set(
            self.fighting_game.player.x(),
            self.fighting_game.player.y(),
        )
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        let interpolation = self.fixed_timestep.interpolation();

        self.draw_ground(context)?;
        if self.should_draw_debug_text {
            self.draw_debug_text(context)?;
        }
        self.draw_character(context, interpolation)?;

        Ok(())
    }

    fn screen_width(&self, context: &mut Context) -> f32 { graphics::screen_coordinates(context).w }
    fn screen_height(&self, context: &mut Context) -> f32 { graphics::screen_coordinates(context).h }
    fn camera_pixel_x(&self, context: &mut Context) -> f32 { 0.5 * self.screen_width(context) }
    fn camera_pixel_y(&self, context: &mut Context) -> f32 { 0.5 * self.screen_height(context) + 100.0 }

    fn draw_character(&mut self, context: &mut Context, interpolation: f32) -> GameResult {
        let character_pixel_width = 50.0;
        let character_pixel_height = 100.0;
        let character_pixel_x = self.character_position.x(interpolation) * self.camera_zoom + self.camera_pixel_x(context) - 0.5 * character_pixel_width;
        let character_pixel_y = -self.character_position.y(interpolation) * self.camera_zoom + self.camera_pixel_y(context) - character_pixel_height;
        let character = Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            Rect::new(character_pixel_x, character_pixel_y, character_pixel_width, character_pixel_height),
            graphics::WHITE,
        )?;
        graphics::draw(context, &character, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_ground(&mut self, context: &mut Context) -> GameResult {
        let camera_pixel_y = self.camera_pixel_y(context);
        let screen_width = self.screen_width(context);
        let ground = Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            Rect::new(0.0, camera_pixel_y, screen_width, 2000.0),
            graphics::Color::new(0.3, 0.3, 0.3, 1.0),
        )?;
        graphics::draw(context, &ground, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn update_debug_text(&mut self) {
        self.debug_text.update_text(
            self.fighting_game.player.state_as_string(),
            format!("{}", self.fighting_game.player.state_frame()),
            format!("{:.5}", self.fighting_game.player.x_velocity()),
            format!("{:.5}", self.fighting_game.player.y_velocity()),
            format!("{:.4}", self.fighting_game.input.left_stick.x_axis.value()),
            format!("{:.4}", self.fighting_game.input.left_stick.y_axis.value())
        );
    }

    fn draw_debug_text(&mut self, context: &mut Context) -> GameResult {
        let screen_width = self.screen_width(context);
        let screen_height = self.screen_height(context);
        self.debug_text.draw(context, 0.5 * screen_width + 20.0, 0.5 * screen_height + 200.0)?;
        Ok(())
    }
}
