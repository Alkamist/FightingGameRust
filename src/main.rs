mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod fixed_timestep;
mod fighting_game;
mod fighter;
//mod debug_text;
mod rendered_fighting_game;
mod interpolated_position;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::Instant;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::{self, *};
use piston::window::WindowSettings;

use crate::rendered_fighting_game::RenderedFightingGame;
use crate::controller_state::ControllerState;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("fighting-game", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut input = ControllerState::new(0.2875);
    let mut app = RenderedFightingGame::new(GlGraphics::new(opengl));

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

    let mut events = Events::new(EventSettings::new());
    // If you do set_ups(0) then the rendering lags terribly for some reason.
    events.set_ups(60);
    events.set_max_fps(300);

    let mut time_previous = Instant::now();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    input::Button::Keyboard(key) => match key {
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
                    input::Button::Keyboard(key) => match key {
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
        app.update(delta, &input);

        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}