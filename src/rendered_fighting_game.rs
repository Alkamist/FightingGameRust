use std::time::Duration;
use std::path::Path;

use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;
use graphics::*;

use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;
use crate::interpolated_position::InterpolatedPosition;
use crate::controller_state::ControllerState;

pub struct RenderedFightingGame {
    gl: GlGraphics,
    fighting_game: FightingGame,
    fixed_timestep: FixedTimestep,
    camera_zoom: f64,
    character_position: InterpolatedPosition,
    //text_font: GlyphCache<'static>,
}

impl RenderedFightingGame {
    pub fn new(gl: GlGraphics) -> RenderedFightingGame {
        const GAME_FPS: f64 = 60.0;
        RenderedFightingGame {
            gl: gl,
            fighting_game: FightingGame::new(),
            fixed_timestep: FixedTimestep::new(GAME_FPS),
            camera_zoom: 6.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
            //text_font: GlyphCache::new(&Path::new("assets/consola.ttf")).unwrap(),
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

    pub fn render(&mut self, args: &RenderArgs) {
        let interpolation = self.fixed_timestep.interpolation();

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

        let screen_width = args.window_size[0];
        let screen_height = args.window_size[1];
        let camera_pixel_x = 0.5 * screen_width;
        let camera_pixel_y = 0.5 * screen_height + 130.0;

        let character_pixel_width = 50.0;
        let character_pixel_height = 100.0;
        let character_pixel_x = self.character_position.x(interpolation) * self.camera_zoom + camera_pixel_x - 0.5 * character_pixel_width;
        let character_pixel_y = -self.character_position.y(interpolation) * self.camera_zoom + camera_pixel_y - character_pixel_height;

        let ground_rect = rectangle::rectangle_by_corners(0.0, 0.0, screen_width, 50.0);
        let character_rect = rectangle::rectangle_by_corners(0.0, 0.0, character_pixel_width, character_pixel_height);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            rectangle(GRAY, ground_rect, c.transform.trans(0.0, camera_pixel_y), gl);
            rectangle(WHITE, character_rect, c.transform.trans(character_pixel_x, character_pixel_y), gl);

            let mut text = Text::new(22);
            text.draw(&format!("{}", self.fighting_game.player.x()),
                               &mut self.font,
                               &c.draw_state,
                               c.transform.trans(camera_pixel_x, camera_pixel_y + 250.0),
                               gl);
        });
    }

    fn screen_width(&self, args: &RenderArgs) -> f64 { args.window_size[0] }
    fn screen_height(&self, args: &RenderArgs) -> f64 { args.window_size[1] }
}
