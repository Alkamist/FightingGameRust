use piston_window::*;

use crate::general_math;
use crate::line_math::PolyLine;
use crate::fighter::Fighter;
use crate::fighting_game::FightingGame;

pub struct FightingGameRenderer {
    pub camera_zoom: f64,
    pub camera_x: f64,
    pub camera_y: f64,
    pub middle_mouse_is_down: bool,
    pub glyphs: Glyphs,
}

impl FightingGameRenderer {
    pub fn to_piston_window(window: &mut PistonWindow) -> FightingGameRenderer {
        FightingGameRenderer {
            camera_zoom: 4.0,
            camera_x: 0.0,
            camera_y: 20.0,
            middle_mouse_is_down: false,
            glyphs: window.load_font("C:/Windows/Fonts/consola.ttf").unwrap(),
        }
    }

    pub fn handle_mouse_pan_and_zoom(&mut self, event: &Event) {
        if let Some(args) = event.mouse_scroll_args() {
            let scroll_direction = args[1];
            self.camera_zoom += scroll_direction;
            self.camera_zoom = self.camera_zoom.max(1.0);
        }

        if let Some(args) = event.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    Button::Mouse(button) => match button {
                        MouseButton::Middle => self.middle_mouse_is_down = true,
                        _ => ()
                    },
                    _ => ()
                },
                ButtonState::Release => match args.button {
                    Button::Mouse(button) => match button {
                        MouseButton::Middle => self.middle_mouse_is_down = false,
                        _ => ()
                    },
                    _ => ()
                }
            }
        }

        if let Some(args) = event.mouse_relative_args() {
            if self.middle_mouse_is_down {
                self.camera_x += args[0] / self.camera_zoom;
                self.camera_y += args[1] / self.camera_zoom;
            }
        }
    }

    fn game_x_to_screen_x(&self, value: f64, window_width: f64) -> f64 {
        value * self.camera_zoom + 0.5 * window_width + self.camera_x * self.camera_zoom
    }
    fn game_y_to_screen_y(&self, value: f64, window_height: f64) -> f64 {
        -value * self.camera_zoom + 0.5 * window_height + self.camera_y * self.camera_zoom
    }

    pub fn render(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        device: &mut gfx_device_gl::Device,
        game: &FightingGame,
        window_width: f64,
        window_height: f64,
        interpolation: f64
    ) {
        let interpolation = if game.is_paused { 1.0 } else { interpolation };
        clear([0.0, 0.0, 0.0, 1.0], graphics);
        self.draw_collision_lines(context, graphics, &game.collision_poly_lines, window_width, window_height);
        self.draw_character(context, graphics, &game.player, window_width, window_height, interpolation);
        self.draw_debug_text(context, graphics, device, &game, window_width, window_height);
    }

    fn draw_character(
        &self,
        context: Context,
        graphics: &mut G2d,
        character: &Fighter,
        window_width: f64,
        window_height: f64,
        interpolation: f64,
    ) {
        // Draw the main character body.
        let character_interpolated_x = general_math::lerp(character.previous_position.x, character.position.x, interpolation);
        let character_interpolated_y = general_math::lerp(character.previous_position.y, character.position.y, interpolation);
        let character_pixel_x = self.game_x_to_screen_x(character_interpolated_x, window_width);
        let character_pixel_y = self.game_y_to_screen_y(character_interpolated_y, window_height);
        let screen_ecb = [
            [self.camera_zoom * (character.ecb.bottom.x), self.camera_zoom * -(character.ecb.bottom.y)],
            [self.camera_zoom * (character.ecb.left.x), self.camera_zoom * -(character.ecb.left.y)],
            [self.camera_zoom * (character.ecb.top.x), self.camera_zoom * -(character.ecb.top.y)],
            [self.camera_zoom * (character.ecb.right.x), self.camera_zoom * -(character.ecb.right.y)]
        ];
        polygon(
            [0.5, 0.5, 0.5, 1.0],
            &screen_ecb,
            context.transform.trans(character_pixel_x, character_pixel_y),
            graphics,
        );

        // Draw a way to tell which way the character is facing.
        let character_facing_width = 2.0 * self.camera_zoom;
        let character_facing_offset = (character_facing_width + 6.0) * character.facing_direction();
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
        game: &FightingGame,
        window_width: f64,
        window_height: f64,
    ) {
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
            &format!("{}", game.player.state_frame)[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.5}", game.player.velocity.x)[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.5}", game.player.velocity.y)[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.4}", game.input.x_axis.value)[..],
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(offset + debug_text_pixel_x - 2.0 * x_spacing, debug_text_pixel_y + y_spacing),
            graphics,
        ).unwrap();

        Text::new_color(color, 20).draw(
            &format!("{:.4}", game.input.y_axis.value)[..],
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
        poly_line: &PolyLine,
    ) {
        for line_segment in &poly_line.segments {
            let color = {
                //if collision_line.is_ground() { [0.3, 0.3, 0.3, 1.0] }
                //else { [0.9, 0.3, 0.3, 1.0] }
                [0.3, 0.3, 0.3, 1.0]
            };

            let left_point = line_segment.left_point();
            let right_point = line_segment.right_point();

            line(
                color,
                1.0,
                [
                    self.game_x_to_screen_x(left_point.x, window_width),
                    self.game_y_to_screen_y(left_point.y, window_height),
                    self.game_x_to_screen_x(right_point.x, window_width),
                    self.game_y_to_screen_y(right_point.y, window_height),
                ],
                context.transform.trans(0.0, 0.0),
                graphics,
            );
        }
    }

    fn draw_collision_lines(
        &self,
        context: Context,
        graphics: &mut G2d,
        collision_lines: &Vec<PolyLine>,
        window_width: f64,
        window_height: f64,
    ) {
        for poly_line in collision_lines {
            self.draw_poly_line(context, graphics, window_width, window_height, &poly_line);
        }
    }
}
