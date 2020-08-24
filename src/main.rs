mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod fixed_timestep;
mod fighting_game;
mod fighter;
mod rendered_fighting_game;
mod interpolated_position;

extern crate find_folder;
extern crate piston_window;
use piston_window::*;

use std::time::Instant;

use crate::controller_state::ControllerState;
use crate::rendered_fighting_game::RenderedFightingGame;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fighting Game", [800, 600]).build().unwrap();
    window.set_max_fps(300);
    window.set_ups(60);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("consola.ttf")).unwrap();

    let mut game = RenderedFightingGame::new();

    let mut input = ControllerState::new(0.2875);
    let mut left_state = false;
    let mut right_state = false;
    let mut down_state = false;
    let mut up_state = false;
    let mut x_state = false;
    let mut y_state = false;
    let mut z_state = false;
    let mut l_state = false;
    let mut r_state = false;
    let mut start_state = false;

    const DEBUG_TEXT_COLOR: [f32; 4] = [0.2, 0.9, 0.2, 1.0];
    const DEBUG_TEXT_OFFSET: f64 = 50.0;
    const DEBUG_TEXT_X_SPACING: f64 = 150.0;
    const DEBUG_TEXT_Y_SPACING: f64 = 25.0;

    let mut time_previous = Instant::now();
    while let Some(e) = window.next() {
        if let Some(args) = e.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => left_state = true,
                        Key::D => right_state = true,
                        Key::S => down_state = true,
                        Key::W => up_state = true,
                        Key::Backslash => x_state = true,
                        Key::LeftBracket => y_state = true,
                        Key::Equals => z_state = true,
                        Key::Semicolon => l_state = true,
                        Key::RightBracket => r_state = true,
                        Key::D5 => start_state = true,
                        _ => ()
                    },
                    _ => ()
                },
                ButtonState::Release => match args.button {
                    Button::Keyboard(key) => match key {
                        Key::A => left_state = false,
                        Key::D => right_state = false,
                        Key::S => down_state = false,
                        Key::W => up_state = false,
                        Key::Backslash => x_state = false,
                        Key::LeftBracket => y_state = false,
                        Key::Equals => z_state = false,
                        Key::Semicolon => l_state = false,
                        Key::RightBracket => r_state = false,
                        Key::D5 => start_state = false,
                        _ => ()
                    },
                    _ => ()
                }
            }
            input.left_stick.x_axis.set_value_from_states(left_state, right_state);
            input.left_stick.y_axis.set_value_from_states(down_state, up_state);
            input.x_button.set_pressed(x_state);
            input.y_button.set_pressed(y_state);
            input.z_button.set_pressed(z_state);
            input.l_button.set_pressed(l_state);
            input.r_button.set_pressed(r_state);
            input.start_button.set_pressed(start_state);
        }

        let time_current = Instant::now();
        let delta = time_current - time_previous;
        time_previous = time_current;
        game.update(delta, &input);

        game.render(&e, &mut window);

        // ============== DRAW DEBUG TEXT ==============

        let debug_text_pixel_x = 0.5 * window.size().width;
        let debug_text_pixel_y = 0.5 * window.size().height + 250.0;
        window.draw_2d(&e, |c, g, device| {
            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &game.player().state_as_string()[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x, debug_text_pixel_y),
                g,
            ).unwrap();

            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &format!("{}", game.player().state_frame())[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x, debug_text_pixel_y + DEBUG_TEXT_Y_SPACING),
                g,
            ).unwrap();

            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &format!("{:.5}", game.player().x_velocity())[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x - DEBUG_TEXT_X_SPACING, debug_text_pixel_y + DEBUG_TEXT_Y_SPACING),
                g,
            ).unwrap();

            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &format!("{:.5}", game.player().y_velocity())[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x - DEBUG_TEXT_X_SPACING, debug_text_pixel_y),
                g,
            ).unwrap();

            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &format!("{:.4}", game.input().left_stick.x_axis.value())[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x - 2.0 * DEBUG_TEXT_X_SPACING, debug_text_pixel_y + DEBUG_TEXT_Y_SPACING),
                g,
            ).unwrap();

            Text::new_color(DEBUG_TEXT_COLOR, 20).draw(
                &format!("{:.4}", game.input().left_stick.y_axis.value())[..],
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(DEBUG_TEXT_OFFSET + debug_text_pixel_x - 2.0 * DEBUG_TEXT_X_SPACING, debug_text_pixel_y),
                g,
            ).unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }
}
