#![windows_subsystem="windows"]

mod game_math;
mod collision;
mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod fixed_timestep;
mod fighting_game;
mod fighter;
mod fighting_stage;
mod rendered_fighting_game;
mod interpolated_position;
mod keyboard_controller;

extern crate gfx_device_gl;
extern crate find_folder;
extern crate piston_window;
use piston_window::*;

use std::time::Instant;

use crate::rendered_fighting_game::RenderedFightingGame;
use crate::keyboard_controller::KeyboardController;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fighting Game", [800, 600]).build().unwrap();
    window.set_max_fps(300);
    window.set_ups(60);

    let mut game = RenderedFightingGame::new(&mut window);
    let mut keyboard_controller = KeyboardController::new(0.2875);

    let mut mouse_middle_is_down = false;

    let mut time_previous = Instant::now();
    while let Some(event) = window.next() {
        keyboard_controller.update(&event);

        if let Some(args) = event.mouse_scroll_args() {
            let scroll_direction = args[1];
            game.set_camera_zoom(game.camera_zoom() + scroll_direction);
        }

        if let Some(args) = event.button_args() {
            match args.state {
                ButtonState::Press => match args.button {
                    Button::Mouse(button) => match button {
                        MouseButton::Middle => mouse_middle_is_down = true,
                        _ => ()
                    },
                    _ => ()
                },
                ButtonState::Release => match args.button {
                    Button::Mouse(button) => match button {
                        MouseButton::Middle => mouse_middle_is_down = false,
                        _ => ()
                    },
                    _ => ()
                }
            }
        }

        if let Some(args) = event.mouse_relative_args() {
            if mouse_middle_is_down {
                game.set_camera_x(game.camera_x() + args[0] / game.camera_zoom());
                game.set_camera_y(game.camera_y() + args[1] / game.camera_zoom());
            }
        }

        let time_current = Instant::now();
        let delta = time_current - time_previous;
        time_previous = time_current;
        game.update(delta, &keyboard_controller.controller_state);

        let window_size = window.size();
        let window_width = window_size.width;
        let window_height = window_size.height;
        window.draw_2d(&event, |context, graphics, device| {
            game.render(context, graphics, device, window_width, window_height);
        });
    }
}
