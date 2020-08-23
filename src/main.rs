mod button;
mod analog_axis;
mod analog_stick;
mod controller_state;
mod fixed_timestep;
mod fighting_game;
mod fighter;
mod debug_text;
mod fighting_game_renderer;
mod interpolated_position;

use ggez::{
    timer,
    Context,
    ContextBuilder,
    GameResult,
    graphics,
    conf,
    input::keyboard,
    event::{
        self,
        EventHandler,
        KeyCode,
    },
};

use crate::controller_state::ControllerState;
//use crate::fixed_timestep::FixedTimestep;
use crate::fighting_game::FightingGame;
use crate::fighting_game_renderer::FightingGameRenderer;

struct MainState {
    //fixed_timestep: FixedTimestep,
    game_fps: u32,
    fighting_game: FightingGame,
    fighting_game_renderer: FightingGameRenderer,
    controller_state: ControllerState,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        const GAME_FPS: u32 = 60;
        let s = MainState {
            //fixed_timestep: FixedTimestep::new(60.0),
            game_fps: GAME_FPS,
            fighting_game: FightingGame::new(),
            fighting_game_renderer: FightingGameRenderer::new(GAME_FPS),
            controller_state: ControllerState::new(0.2875),
        };
        Ok(s)
    }

    fn update_game_input_with_keyboard(&mut self, context: &mut Context) {
        self.controller_state.left_stick.x_axis.set_value_from_states(
            keyboard::is_key_pressed(context, KeyCode::A),
            keyboard::is_key_pressed(context, KeyCode::D)
        );
        self.controller_state.left_stick.y_axis.set_value_from_states(
            keyboard::is_key_pressed(context, KeyCode::S),
            keyboard::is_key_pressed(context, KeyCode::W)
        );

        self.controller_state.x_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::Backslash));
        self.controller_state.y_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::LBracket));
        self.controller_state.z_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::Equals));
        self.controller_state.l_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::Semicolon));
        self.controller_state.r_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::RBracket));

        self.controller_state.start_button.set_pressed(keyboard::is_key_pressed(context, KeyCode::Key5));

        self.controller_state.convert_to_melee_values();
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
//        self.update_game_input_with_keyboard(context);
//
//        let fighting_game = &mut self.fighting_game;
//        let controller_state = &mut self.controller_state;
//        self.fixed_timestep.update(timer::delta(context), || {
//            fighting_game.update(controller_state);
//        });
//
//        self.controller_state.update();

        while timer::check_update_time(context, self.game_fps) {
            self.update_game_input_with_keyboard(context);
            self.fighting_game.update(&self.controller_state);
            self.fighting_game_renderer.on_game_update(&self.fighting_game);
            self.controller_state.update();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        self.fighting_game_renderer.draw(context)?;

        graphics::present(context)?;
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let context_builder = ContextBuilder::new("fighting_game", "corey")
        .window_setup(conf::WindowSetup::default().title("Fighting Game"))
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 960.0));
    let (context, event_loop) = &mut context_builder.build()?;
    let state = &mut MainState::new()?;
    event::run(context, event_loop, state)
}
