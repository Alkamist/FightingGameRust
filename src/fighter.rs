use crate::controller_state::ControllerState;
use crate::button::Button;
use crate::analog_axis::AnalogAxis;

#[derive(Copy, Clone)]
pub enum FighterState {
    Idle,
    Turn,
    Dash
}

pub struct Fighter {
    ground_friction: f32,

    dash_start_velocity: f32,
    dash_max_velocity: f32,
    dash_base_acceleration: f32,
    dash_axis_acceleration: f32,

    walk_start_velocity: f32,
    walk_max_velocity: f32,
    walk_acceleration: f32,

    air_friction: f32,
    air_base_acceleration: f32,
    air_axis_acceleration: f32,
    air_max_velocity: f32,

    jump_squat_frames: u32,
    jump_velocity_dampening: f32,
    jump_max_horizontal_velocity: f32,
    jump_start_horizontal_velocity: f32,
    short_hop_velocity: f32,
    full_hop_velocity: f32,
    fall_velocity: f32,
    fast_fall_velocity: f32,
    air_jump_velocity_multiplier: f32,
    air_jump_horizontal_axis_multiplier: f32,
    air_jumps: u32,
    gravity: f32,

    dash_min_frames: u32,
    dash_max_frames: u32,
    slow_dash_back_frames: u32,
    turn_frames: u32,
    run_turn_frames: u32,
    run_brake_frames: u32,

    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    x_previous: f32,
    y_previous: f32,
    air_jumps_left: u32,
    is_facing_right: bool,
    was_facing_right: bool,

    state: FighterState,
    state_previous: FighterState,
    state_frame: u32,

    input: ControllerState,

    reset_dash_velocity: bool
}

// Character builders.
impl Fighter {
    pub fn fox() -> Fighter {
        Fighter{
            ground_friction: 0.08,
            dash_start_velocity: 1.9,
            dash_max_velocity: 2.2,
            dash_base_acceleration: 0.02,
            dash_axis_acceleration: 0.1,
            walk_start_velocity: 0.16,
            walk_max_velocity: 1.6,
            walk_acceleration: 0.2,
            air_friction: 0.02,
            air_base_acceleration: 0.02,
            air_axis_acceleration: 0.06,
            air_max_velocity: 0.83,
            jump_squat_frames: 3,
            jump_velocity_dampening: 0.83,
            jump_max_horizontal_velocity: 1.7,
            jump_start_horizontal_velocity: 0.72,
            short_hop_velocity: 2.1,
            full_hop_velocity: 3.68,
            fall_velocity: 2.8,
            fast_fall_velocity: 3.4,
            air_jump_velocity_multiplier: 1.2,
            air_jump_horizontal_axis_multiplier: 0.9,
            air_jumps: 1,
            gravity: 0.23,
            dash_min_frames: 11,
            dash_max_frames: 21,
            slow_dash_back_frames: 5,
            turn_frames: 11,
            run_turn_frames: 30,
            run_brake_frames: 18,
            x: 0.0,
            y: 0.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            x_previous: 0.0,
            y_previous: 0.0,
            air_jumps_left: 1,
            is_facing_right: true,
            was_facing_right: true,
            state: FighterState::Idle,
            state_previous: FighterState::Idle,
            state_frame: 0,
            input: ControllerState::new(0.2875),
            reset_dash_velocity: false
        }
    }
}

// Helper methods.
impl Fighter {
    pub fn x_axis(&self) -> &AnalogAxis { &self.input.left_stick.x_axis }
    pub fn y_axis(&self) -> &AnalogAxis { &self.input.left_stick.y_axis }
    fn c_x_axis(&self) -> &AnalogAxis { &self.input.c_stick.x_axis }
    fn c_y_axis(&self) -> &AnalogAxis { &self.input.c_stick.y_axis }
    fn a_button(&self) -> &Button { &self.input.a_button }
    fn b_button(&self) -> &Button { &self.input.b_button }
    fn x_button(&self) -> &Button { &self.input.x_button }
    fn y_button(&self) -> &Button { &self.input.y_button }
    fn z_button(&self) -> &Button { &self.input.z_button }
    fn r_button(&self) -> &Button { &self.input.r_button }
    fn l_button(&self) -> &Button { &self.input.l_button }
    fn start_button(&self) -> &Button { &self.input.start_button }
    fn d_left_button(&self) -> &Button { &self.input.d_left_button }
    fn d_right_button(&self) -> &Button { &self.input.d_right_button }
    fn d_down_button(&self) -> &Button { &self.input.d_down_button }
    fn d_up_button(&self) -> &Button { &self.input.d_up_button }
    fn should_jump(&self) -> bool { self.x_button().just_pressed() || self.y_button().just_pressed() }
    fn jump_is_active(&self) -> bool { self.x_button().is_pressed() || self.y_button().is_pressed() }
    fn x_axis_is_forward(&self) -> bool {
        self.x_axis().is_active() && (self.x_axis().value() > 0.0 && self.is_facing_right
                                   || self.x_axis().value() < 0.0 && !self.is_facing_right)
    }
    fn x_axis_is_backward(&self) -> bool { self.x_axis().is_active() && !self.x_axis_is_forward() }
    fn x_axis_smashed(&self) -> bool { self.x_axis().magnitude() >= 0.8 && self.x_axis().active_frames() < 2 }
    fn y_axis_smashed(&self) -> bool { self.y_axis().magnitude() >= 0.6625 && self.y_axis().active_frames() < 2 }
}

// Public methods.
impl Fighter {
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn x_velocity(&self) -> f32 { self.x_velocity }
    pub fn y_velocity(&self) -> f32 { self.y_velocity }

    pub fn state_frame(&self) -> u32 { self.state_frame }
    pub fn state(&self) -> FighterState { self.state }
    pub fn set_state(&mut self, new_state: FighterState) {
        self.state_frame = 0;
        self.state_previous = self.state;
        self.state = new_state;
    }
    pub fn facing_direction(&self) -> f32 { if self.is_facing_right { 1.0 } else { -1.0 } }
    pub fn just_turned(&self) -> bool { self.is_facing_right != self.was_facing_right }

    pub fn state_as_string(&self) -> String {
        match self.state {
            FighterState::Idle => String::from("Idle"),
            FighterState::Turn => String::from("Turn"),
            FighterState::Dash => String::from("Dash")
        }
    }
}

// State update logic.
impl Fighter {
    pub fn update(&mut self, input: ControllerState) {
        self.input = input;

        self.was_facing_right = self.is_facing_right;
        self.x_previous = self.x;
        self.y_previous = self.y;

        // Handle state transition.
        match self.state {
            FighterState::Idle => self.state_idle_transition(),
            FighterState::Turn => self.state_turn_transition(),
            FighterState::Dash => self.state_dash_transition(),
        }

        // Handle state enter.
        if self.state_frame == 0 {
            match self.state {
                FighterState::Idle => self.state_idle_enter(),
                FighterState::Turn => self.state_turn_enter(),
                FighterState::Dash => self.state_dash_enter(),
            }
        }

        // Handle state update.
        match self.state {
            FighterState::Idle => self.state_idle_update(),
            FighterState::Turn => self.state_turn_update(),
            FighterState::Dash => self.state_dash_update(),
        }

        //println!("{:?}", self.state);

        // Extremely basic ground collision logic for now.
        if self.y < 0.0 {
            self.y = 0.0;
            self.y_velocity = 0.0;
            //land();
        }

        self.state_frame += 1;
    }

    fn move_with_velocity(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
    }

    fn apply_friction(&self, velocity: f32, friction: f32) -> f32 {
        velocity - velocity.signum() * velocity.abs().min(friction)
    }

    fn apply_acceleration(&self,
                          velocity: f32,
                          axis: &AnalogAxis,
                          base_acceleration: f32,
                          axis_acceleration: f32,
                          max_velocity: f32,
                          friction: f32) -> f32 {
        let mut new_velocity = velocity;

        if velocity.abs() > max_velocity {
            new_velocity = self.apply_friction(velocity, friction);
        }

        let mut acceleration = axis.direction() * base_acceleration + axis.value() * axis_acceleration;

        if axis.value() > 0.0 {
            acceleration = acceleration.min(max_velocity - new_velocity);
            acceleration = acceleration.max(0.0);
            new_velocity += acceleration;
        }
        else if axis.value() < 0.0 {
            acceleration = acceleration.max(-max_velocity - new_velocity);
            acceleration = acceleration.min(0.0);
            new_velocity += acceleration;
        }

        new_velocity
    }
}

// Idle.
impl Fighter {
    fn state_idle_transition(&mut self) {
        if self.x_axis_is_forward() && self.x_axis_smashed() {
            self.set_state(FighterState::Dash);
        } else if self.x_axis_is_backward() {
            self.set_state(FighterState::Turn);
        }
    }

    fn state_idle_enter(&mut self) {
        match self.state_previous {
            _ => ()
        }
    }

    fn state_idle_update(&mut self) {
        self.x_velocity = self.apply_friction(self.x_velocity, self.ground_friction);
        self.move_with_velocity();
    }
}

// Turn.
impl Fighter {
    fn state_turn_transition(&mut self) {
        if self.x_axis_is_backward() && self.x_axis_smashed() {
            self.set_state(FighterState::Dash);
        } else if self.state_frame >= self.turn_frames {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_turn_enter(&mut self) {
        match self.state_previous {
            FighterState::Dash => {
                // I'm unsure where the 1.73 here comes from but it is necessary for now.
                self.x_velocity -= self.x_velocity.signum() * 1.73;
            },
            _ => ()
        }
    }

    fn state_turn_update(&mut self) {
        // Not quite right. Turn friction in melee applies on the first frame while walking,
        // but not during the one frame of turn seen while dash dancing. I need to implement that.
        if self.state_frame > 1 {
            self.x_velocity = self.apply_friction(self.x_velocity, self.ground_friction * 2.0);
        }
        if self.x_axis_is_backward() && self.state_frame == self.slow_dash_back_frames {
            self.is_facing_right = self.x_axis().value() >= 0.0;
        }
        self.move_with_velocity();
    }
}

// Dash.
impl Fighter {
    fn state_dash_transition(&mut self) {
        if self.x_axis_is_forward() && self.state_frame >= self.dash_max_frames {
            self.set_state(FighterState::Dash);
        } else if !self.x_axis().is_active() && self.state_frame >= self.dash_max_frames {
            self.set_state(FighterState::Idle);
        } else if self.x_axis_is_backward() {
            self.set_state(FighterState::Turn);
        }
    }

    fn state_dash_enter(&mut self) {
        match self.state_previous {
            FighterState::Turn => { self.is_facing_right = !self.is_facing_right; },
            FighterState::Dash => { self.reset_dash_velocity = true; },
            _ => ()
        }
    }

    fn state_dash_update(&mut self) {
        if self.state_frame == 1 {
            if self.reset_dash_velocity {
                self.x_velocity = 0.0;
                self.reset_dash_velocity = false;
            }

            self.x_velocity += self.dash_start_velocity * self.facing_direction();
            if self.x_velocity.abs() > self.dash_max_velocity {
                self.x_velocity = self.dash_max_velocity * self.facing_direction();
            }
        }
        if self.state_frame >= 1 {
            if !self.x_axis().is_active() {
                self.x_velocity = self.apply_friction(self.x_velocity, self.ground_friction);
            }
            else {
                self.x_velocity = self.apply_acceleration(self.x_velocity,
                                                          self.x_axis(),
                                                          self.dash_base_acceleration,
                                                          self.dash_axis_acceleration,
                                                          self.dash_max_velocity,
                                                          self.ground_friction);
            }
        }
        self.move_with_velocity();
    }
}

