#![windows_subsystem="windows"]

mod point_math;
mod vector_math;
mod line_math;
mod button;
mod analog_axis;
mod controller_state;
mod digital_input;
mod fighter;

extern crate gfx_device_gl;
extern crate find_folder;
extern crate piston_window;
use piston_window::*;

use std::time::Instant;

use crate::digital_input::DigitalInput;
use crate::controller_state::ControllerState;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fighting Game", [800, 600]).build().unwrap();
    window.set_max_fps(300);
    window.set_ups(60);

    //let mut game = RenderedFightingGame::new(&mut window);
    let mut digital_input = DigitalInput::default();
    let mut controller_state = ControllerState::default();

    //let mut mouse_middle_is_down = false;

    let mut time_previous = Instant::now();
    while let Some(event) = window.next() {
        digital_input.update_states_with_piston_window_event(&event);
        digital_input.update_controller_state(&mut controller_state);

        //if let Some(args) = event.mouse_scroll_args() {
        //    let scroll_direction = args[1];
        //    game.set_camera_zoom(game.camera_zoom() + scroll_direction);
        //}

        //if let Some(args) = event.button_args() {
        //    match args.state {
        //        ButtonState::Press => match args.button {
        //            Button::Mouse(button) => match button {
        //                MouseButton::Middle => mouse_middle_is_down = true,
        //                _ => ()
        //            },
        //            _ => ()
        //        },
        //        ButtonState::Release => match args.button {
        //            Button::Mouse(button) => match button {
        //                MouseButton::Middle => mouse_middle_is_down = false,
        //                _ => ()
        //            },
        //            _ => ()
        //        }
        //    }
        //}

        //if let Some(args) = event.mouse_relative_args() {
        //    if mouse_middle_is_down {
        //        game.set_camera_x(game.camera_x() + args[0] / game.camera_zoom());
        //        game.set_camera_y(game.camera_y() + args[1] / game.camera_zoom());
        //    }
        //}

        let time_current = Instant::now();
        let delta = time_current - time_previous;
        time_previous = time_current;
        //game.update(delta, &keyboard_controller.controller_state);

        let window_size = window.size();
        let window_width = window_size.width;
        let window_height = window_size.height;
        //window.draw_2d(&event, |context, graphics, device| {
        //    game.render(context, graphics, device, window_width, window_height);
        //});
    }
}
