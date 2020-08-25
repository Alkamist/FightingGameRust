use piston_window::*;

use crate::fighting_game::FightingGame;

pub struct DebugText {
    glyphs: Glyphs,
    color: [f32; 4],
    x_spacing: f64,
    y_spacing: f64,
    offset: f64,
}

impl DebugText {
    pub fn new(window: &mut PistonWindow) -> DebugText {
        DebugText {
            glyphs: window.load_font("C:/Windows/Fonts/consola.ttf").unwrap(),
            color: [0.2, 0.9, 0.2, 1.0],
            x_spacing: 150.0,
            y_spacing: 25.0,
            offset: 50.0,
        }
    }

    pub fn draw(&mut self, game: &FightingGame, window: &mut PistonWindow, event: &Event) {
        let debug_text_pixel_x = 0.5 * window.size().width;
        let debug_text_pixel_y = 0.5 * window.size().height + 250.0;

        let glyphs = &mut self.glyphs;
        let color = self.color;
        let offset = self.offset;
        let x_spacing = self.x_spacing;
        let y_spacing = self.y_spacing;

        window.draw_2d(event, |c, g, device| {
            Text::new_color(color, 20).draw(
                &game.player.state_as_string()[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x, debug_text_pixel_y),
                g,
            ).unwrap();

            Text::new_color(color, 20).draw(
                &format!("{}", game.player.state_frame())[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x, debug_text_pixel_y + y_spacing),
                g,
            ).unwrap();

            Text::new_color(color, 20).draw(
                &format!("{:.5}", game.player.x_velocity())[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y + y_spacing),
                g,
            ).unwrap();

            Text::new_color(color, 20).draw(
                &format!("{:.5}", game.player.y_velocity())[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x - x_spacing, debug_text_pixel_y),
                g,
            ).unwrap();

            Text::new_color(color, 20).draw(
                &format!("{:.4}", game.input.left_stick.x_axis.value())[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x - 2.0 * x_spacing, debug_text_pixel_y + y_spacing),
                g,
            ).unwrap();

            Text::new_color(color, 20).draw(
                &format!("{:.4}", game.input.left_stick.y_axis.value())[..],
                glyphs,
                &c.draw_state,
                c.transform.trans(offset + debug_text_pixel_x - 2.0 * x_spacing, debug_text_pixel_y),
                g,
            ).unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }
}
