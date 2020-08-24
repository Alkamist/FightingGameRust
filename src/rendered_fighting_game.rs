use std::time::Duration;

use piston_window::*;

use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;
use crate::interpolated_position::InterpolatedPosition;
use crate::controller_state::ControllerState;

pub struct RenderedFightingGame {
    fighting_game: FightingGame,
    fixed_timestep: FixedTimestep,
    camera_zoom: f64,
    character_position: InterpolatedPosition,
}

impl RenderedFightingGame {
    pub fn new() -> RenderedFightingGame {
        const GAME_FPS: f64 = 60.0;
        RenderedFightingGame {
            fighting_game: FightingGame::new(),
            fixed_timestep: FixedTimestep::new(GAME_FPS),
            camera_zoom: 6.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
        }
    }

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

    pub fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        let interpolation = self.fixed_timestep.interpolation();

        const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const CHARACTER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GROUND_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

        let window_size = window.size();
        let screen_width = window_size.width;
        let screen_height = window_size.height;
        let camera_pixel_x = 0.5 * screen_width;
        let camera_pixel_y = 0.5 * screen_height + 130.0;

        let character_pixel_width = 50.0;
        let character_pixel_height = 100.0;
        let character_pixel_x = self.character_position.x(interpolation) * self.camera_zoom + camera_pixel_x - 0.5 * character_pixel_width;
        let character_pixel_y = -self.character_position.y(interpolation) * self.camera_zoom + camera_pixel_y - character_pixel_height;

        let ground_rect = rectangle::rectangle_by_corners(0.0, 0.0, screen_width, 50.0);
        let character_rect = rectangle::rectangle_by_corners(0.0, 0.0, character_pixel_width, character_pixel_height);

        window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);

            rectangle(GROUND_COLOR, ground_rect, c.transform.trans(0.0, camera_pixel_y), g);
            rectangle(CHARACTER_COLOR, character_rect, c.transform.trans(character_pixel_x, character_pixel_y), g);

            //let mut text = Text::new(22);
            //text.draw(&format!("{}", self.fighting_game.player.x()),
            //                   &mut self.font,
            //                   &c.draw_state,
            //                   c.transform.trans(camera_pixel_x, camera_pixel_y + 250.0),
            //                   gl);
        });
    }
}
