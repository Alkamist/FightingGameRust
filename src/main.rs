mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod keyboard;
mod fixed_timestep;
mod fighting_game;
mod fighter;

use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{graphics, nalgebra as na, timer};
use ggez::{Context, GameResult};

use crate::keyboard::{Keyboard, KeyboardKey, keyboard_key_from_ggez_key_code};
use crate::controller_state::ControllerState;
use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;

struct MainState {
    keyboard: Keyboard,
    fixed_timestep: FixedTimestep,
    fighting_game: FightingGame,
    controller_state: ControllerState,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            keyboard: Keyboard::new(),
            fixed_timestep: FixedTimestep::new(60.0),
            fighting_game: FightingGame::new(),
            controller_state: ControllerState::new(0.2875),
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.controller_state.left_stick.x_axis.set_value_from_states(
            self.keyboard.key(KeyboardKey::A).is_pressed(),
            self.keyboard.key(KeyboardKey::D).is_pressed()
        );

        let fighting_game = &mut self.fighting_game;
        let controller_state = &mut self.controller_state;
        self.fixed_timestep.update(timer::delta(context), || { fighting_game.update(controller_state); });

        self.controller_state.update();
        self.keyboard.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.0, 1.0].into());

        let screen_coordinates = graphics::screen_coordinates(context);
        let screen_width = screen_coordinates.w;
        let screen_height = screen_coordinates.h;

        let camera_zoom = 6.0;
        let camera_pixel_x = 0.5 * screen_width;
        let camera_pixel_y = 0.5 * screen_height + 100.0;
        let character_pixel_width = 50.0;
        let character_pixel_height = 50.0;
        let character_pixel_x = self.fighting_game.player.x() * camera_zoom + camera_pixel_x - 0.5 * character_pixel_width;
        let character_pixel_y = self.fighting_game.player.y() * camera_zoom + camera_pixel_y - character_pixel_height;

        let character = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(character_pixel_x, character_pixel_y, character_pixel_width, character_pixel_height),
            graphics::WHITE,
        )?;
        graphics::draw(context, &character, (na::Point2::new(0.0, 0.0),))?;

        let ground = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(0.0, camera_pixel_y, screen_width, 2000.0),
            graphics::Color::new(0.5, 0.5, 0.5, 1.0),
        )?;
        graphics::draw(context, &ground, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(context)?;
        Ok(())
    }

    fn key_down_event(&mut self, _context: &mut Context, key_code: KeyCode, _mods: KeyMods, _repeat: bool) {
        self.keyboard.key(keyboard_key_from_ggez_key_code(key_code)).set_pressed(true)
    }

    fn key_up_event(&mut self, _context: &mut Context, key_code: KeyCode, _mods: KeyMods) {
        self.keyboard.key(keyboard_key_from_ggez_key_code(key_code)).set_pressed(false)
    }
}

pub fn main() -> ggez::GameResult {
    let context_builder = ggez::ContextBuilder::new("fighting_game", "corey");
    let (context, event_loop) = &mut context_builder.build()?;
    let state = &mut MainState::new()?;
    event::run(context, event_loop, state)
}