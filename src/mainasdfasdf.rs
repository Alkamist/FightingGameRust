mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod keyboard;
mod fixed_timestep;
mod fighting_game;
mod fighter;

use std::time::{Instant, Duration};

use crate::controller_state::ControllerState;
use crate::fighting_game::FightingGame;
use crate::fixed_timestep::FixedTimestep;

pub fn main() {
    let mut delta = Duration::new(0, 0);
    let mut time0 = Instant::now();
    let mut frames = 0u64;
    let mut fixed_timestep = FixedTimestep::new(60.0);
    let mut input = ControllerState::new(0.2875);
    let mut game = FightingGame::new();
    loop {
        let time1 = Instant::now();
        let mut delta = time1 - time0;
        time0 = time1;
        fixed_timestep.update(delta, || {
            game.update(&input);
            input.update();
            frames += 1;
            if frames % 60 == 0 {
                println!("{}", frames);
               // println!("{}", now.elapsed().as_secs_f32());
            }
        })
    }
}