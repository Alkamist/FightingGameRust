#![windows_subsystem="windows"]

mod general_math;
mod point_math;
mod vector_math;
mod line_math;
mod fixed_timestep;
mod button;
mod analog_axis;
mod controller_state;
mod digital_input;
mod fighter;
mod ecb;
mod fighting_game;
mod fighting_game_renderer;

extern crate gfx_device_gl;
extern crate find_folder;
extern crate piston_window;
use piston_window::*;

use std::time::Instant;

use crate::digital_input::DigitalInput;
use crate::controller_state::ControllerState;
use crate::fighting_game::FightingGame;
use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game_renderer::FightingGameRenderer;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fighting Game", [800, 600]).build().unwrap();
    window.set_max_fps(300);
    window.set_ups(60);

    let mut digital_input = DigitalInput::default();
    let mut controller_state = ControllerState::default();
    let mut fighting_game = FightingGame::new();
    let mut fighting_game_renderer = FightingGameRenderer::to_piston_window(&mut window);
    let mut fixed_timestep = FixedTimestep::with_fixed_fps(60.0);

    let mut time_previous = Instant::now();
    while let Some(event) = window.next() {
        digital_input.update_states_with_piston_window_event(&event);
        digital_input.update_controller_state(&mut controller_state);

        let time_current = Instant::now();
        let delta = time_current - time_previous;
        time_previous = time_current;
        fixed_timestep.update(delta, || fighting_game.update(&controller_state));

        fighting_game_renderer.handle_mouse_pan_and_zoom(&event);

        let window_size = window.size();
        let window_width = window_size.width;
        let window_height = window_size.height;
        window.draw_2d(&event, |context, graphics, device| {
            fighting_game_renderer.render(
                context,
                graphics,
                device,
                &fighting_game,
                window_width,
                window_height,
                fixed_timestep.interpolation,
            );
        });
    }
}
