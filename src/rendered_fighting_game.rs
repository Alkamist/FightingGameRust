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
    camera_x: f64,
    camera_y: f64,
    character_position: InterpolatedPosition,
    glyphs: Glyphs,
}

impl RenderedFightingGame {
    pub fn new(window: &mut PistonWindow) -> RenderedFightingGame {
        const GAME_FPS: f64 = 60.0;
        RenderedFightingGame {
            fighting_game: FightingGame::new(),
            fixed_timestep: FixedTimestep::new(GAME_FPS),
            camera_zoom: 4.0,
            camera_x: 0.0,
            camera_y: 20.0,
            character_position: InterpolatedPosition::new(0.0, 0.0),
            glyphs: window.load_font("C:/Windows/Fonts/consola.ttf").unwrap(),
        }
    }

    pub fn camera_x(&self) -> f64 { self.camera_x }
    pub fn set_camera_x(&mut self, value: f64) { self.camera_x = value; }

    pub fn camera_y(&self) -> f64 { self.camera_y }
    pub fn set_camera_y(&mut self, value: f64) { self.camera_y = value; }

    pub fn camera_zoom(&self) -> f64 { self.camera_zoom }
    pub fn set_camera_zoom(&mut self, value: f64) {
        self.camera_zoom = value;
        self.camera_zoom = self.camera_zoom.max(1.0);
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
        value * self.camera_zoom + 0.5 * window_width + self.camera_x * self.camera_zoom
    }
    fn to_screen_y(&self, value: f64, window_height: f64) -> f64 {
        -value * self.camera_zoom + 0.5 * window_height + self.camera_y * self.camera_zoom
    }

    pub fn render(&mut self, context: Context, graphics: &mut G2d, device: &mut gfx_device_gl::Device, window_width: f64, window_height: f64) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);
        self.draw_stage(context, graphics, window_width, window_height);
        self.draw_character(context, graphics, window_width, window_height);
        self.draw_debug_text(context, graphics, device, window_width, window_height);
    }

    fn draw_character(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        window_width: f64,
        window_height: f64,
    ) {
        let interpolation = self.fixed_timestep.interpolation();

        // Draw the main character body.
        let character_pixel_x = self.to_screen_x(self.character_position.x(interpolation), window_width);
        let character_pixel_y = self.to_screen_y(self.character_position.y(interpolation), window_height);
        let player_ecb = self.player().ecb();
        let screen_ecb = [
            [self.camera_zoom * (player_ecb[0][0] as f64), self.camera_zoom * -(player_ecb[0][1] as f64)],
            [self.camera_zoom * (player_ecb[1][0] as f64), self.camera_zoom * -(player_ecb[1][1] as f64)],
            [self.camera_zoom * (player_ecb[2][0] as f64), self.camera_zoom * -(player_ecb[2][1] as f64)],
            [self.camera_zoom * (player_ecb[3][0] as f64), self.camera_zoom * -(player_ecb[3][1] as f64)]
        ];
        polygon(
            [0.5, 0.5, 0.5, 1.0],
            &screen_ecb,
            context.transform.trans(character_pixel_x, character_pixel_y),
            graphics,
        );

        // Draw a way to tell which way the character is facing.
        let character_facing_width = 2.0 * self.camera_zoom;
        let character_facing_offset = (character_facing_width) * (self.player().facing_direction() as f64);
        rectangle(
            [0.9, 0.9, 0.9, 1.0],
            rectangle::rectangle_by_corners(0.0, 0.0, character_facing_width, character_facing_width),
            context.transform.trans(character_pixel_x + character_facing_offset - 0.5 * character_facing_width, character_pixel_y - 12.0 * self.camera_zoom),
            graphics,
        );
    }

    fn draw_debug_text(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        device: &mut gfx_device_gl::Device,
        window_width: f64,
        window_height: f64,
    ) {
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

    fn draw_poly_line(
        &self,
        context: Context,
        graphics: &mut G2d,
        window_width: f64,
        window_height: f64,
        poly_line: &Vec<[f64; 2]>,
        color: types::Color,
    ) {
        let poly_line_length = poly_line.len();
        if poly_line_length > 1 {
            for i in 1..poly_line_length {
                let i_previous = i - 1;
                line(
                    color,
                    1.0,
                    [
                        self.to_screen_x(poly_line[i_previous][0] as f64, window_width),
                        self.to_screen_y(poly_line[i_previous][1] as f64, window_height),
                        self.to_screen_x(poly_line[i][0] as f64, window_width),
                        self.to_screen_y(poly_line[i][1] as f64, window_height),
                    ],
                    context.transform.trans(0.0, 0.0),
                    graphics,
                );
            }
        }
    }

    fn draw_stage(
        &self,
        context: Context,
        graphics: &mut G2d,
        window_width: f64,
        window_height: f64,
    ) {
        let color = [0.3, 0.3, 0.3, 1.0];

        for wall in self.fighting_game.stage.left_walls() {
            self.draw_poly_line(context, graphics, window_width, window_height, wall, color);
        }
        for wall in self.fighting_game.stage.right_walls() {
            self.draw_poly_line(context, graphics, window_width, window_height, wall, color);
        }
        for ground in self.fighting_game.stage.grounds() {
            self.draw_poly_line(context, graphics, window_width, window_height, ground, color);
        }
        for ceiling in self.fighting_game.stage.ceilings() {
            self.draw_poly_line(context, graphics, window_width, window_height, ceiling, color);
        }
        for platform in self.fighting_game.stage.platforms() {
            self.draw_poly_line(context, graphics, window_width, window_height, platform, color);
        }
    }
}
