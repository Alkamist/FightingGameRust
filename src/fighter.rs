use crate::controller_state::ControllerState;
use crate::button::Button;
use crate::analog_axis::AnalogAxis;
use crate::collision::ECB;
use crate::game_math::*;

pub struct Fighter {
    ground_friction: f64,

    dash_start_velocity: f64,
    dash_max_velocity: f64,
    dash_base_acceleration: f64,
    dash_axis_acceleration: f64,

    walk_start_velocity: f64,
    walk_max_velocity: f64,
    walk_acceleration: f64,

    air_friction: f64,
    air_base_acceleration: f64,
    air_axis_acceleration: f64,
    air_max_velocity: f64,

    jump_squat_frames: u32,
    jump_velocity_dampening: f64,
    jump_max_horizontal_velocity: f64,
    jump_start_horizontal_velocity: f64,
    short_hop_velocity: f64,
    full_hop_velocity: f64,
    fall_velocity: f64,
    fast_fall_velocity: f64,
    air_jump_velocity_multiplier: f64,
    air_jump_horizontal_axis_multiplier: f64,
    air_jumps: u32,
    gravity: f64,

    dash_min_frames: u32,
    dash_max_frames: u32,
    slow_dash_back_frames: u32,
    turn_frames: u32,
    run_brake_frames: u32,

    position: Point2D,
    position_previous: Point2D,
    velocity: Vector2D,

    //x: f64,
    //y: f64,
    //x_velocity: f64,
    //y_velocity: f64,
    //x_previous: f64,
    //y_previous: f64,

    air_jumps_left: u32,
    is_facing_right: bool,
    was_facing_right: bool,

    state: FighterState,
    state_previous: FighterState,
    state_frame: u32,

    input: ControllerState,

    dash_should_reset_velocity: bool,
    run_turn_melee_frame: u32,
    run_turn_was_facing_right_initially: bool,
    run_turn_has_changed_direction: bool,
    run_turn_has_fully_turned: bool,

    ecb: ECB,
    ground_line: Option<LineSegment2D>,
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
            run_brake_frames: 18,
            position: Point2D::new(0.0, 0.0),
            position_previous: Point2D::new(0.0, 0.0),
            velocity: Vector2D::new(0.0, 0.0),
            air_jumps_left: 1,
            is_facing_right: true,
            was_facing_right: true,
            state: FighterState::Airborne,
            state_previous: FighterState::Airborne,
            state_frame: 0,
            input: ControllerState::new(0.2875),
            dash_should_reset_velocity: false,
            run_turn_melee_frame: 0,
            run_turn_was_facing_right_initially: true,
            run_turn_has_changed_direction: false,
            run_turn_has_fully_turned: false,
            ecb: ECB::new(
                Point2D::new(0.0, 0.0),
                Point2D::new(-2.3, 6.0),
                Point2D::new(0.0, 12.0),
                Point2D::new(2.3, 6.0),
            ),
            ground_line: None,
        }
    }
}

// Helper methods.
impl Fighter {
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
    pub fn ecb(&self) -> &ECB { &self.ecb }

    pub fn position(&self) -> Point2D { self.position }

    pub fn x(&self) -> f64 { self.position.x() }
    pub fn set_x(&mut self, value: f64) { self.position.set_x(value); }
    pub fn y(&self) -> f64 { self.position.y() }
    pub fn set_y(&mut self, value: f64) { self.position.set_y(value); }
    pub fn x_previous(&self) -> f64 { self.position_previous.x() }
    pub fn y_previous(&self) -> f64 { self.position_previous.y() }
    pub fn x_velocity(&self) -> f64 { self.velocity.x() }
    pub fn set_x_velocity(&mut self, value: f64) { self.velocity.set_x(value); }
    pub fn y_velocity(&self) -> f64 { self.velocity.y() }
    pub fn set_y_velocity(&mut self, value: f64) { self.velocity.set_y(value); }

    pub fn x_axis(&self) -> &AnalogAxis { &self.input.left_stick.x_axis }
    pub fn y_axis(&self) -> &AnalogAxis { &self.input.left_stick.y_axis }
    pub fn c_x_axis(&self) -> &AnalogAxis { &self.input.c_stick.x_axis }
    pub fn c_y_axis(&self) -> &AnalogAxis { &self.input.c_stick.y_axis }
    pub fn a_button(&self) -> &Button { &self.input.a_button }
    pub fn b_button(&self) -> &Button { &self.input.b_button }
    pub fn x_button(&self) -> &Button { &self.input.x_button }
    pub fn y_button(&self) -> &Button { &self.input.y_button }
    pub fn z_button(&self) -> &Button { &self.input.z_button }
    pub fn r_button(&self) -> &Button { &self.input.r_button }
    pub fn l_button(&self) -> &Button { &self.input.l_button }
    pub fn start_button(&self) -> &Button { &self.input.start_button }
    pub fn d_left_button(&self) -> &Button { &self.input.d_left_button }
    pub fn d_right_button(&self) -> &Button { &self.input.d_right_button }
    pub fn d_down_button(&self) -> &Button { &self.input.d_down_button }
    pub fn d_up_button(&self) -> &Button { &self.input.d_up_button }

    pub fn state_frame(&self) -> u32 { self.state_frame }
    pub fn state(&self) -> FighterState { self.state }
    pub fn set_state(&mut self, new_state: FighterState) {
        self.state_frame = 0;
        self.state_previous = self.state;
        self.state = new_state;
    }
    pub fn facing_direction(&self) -> f64 { if self.is_facing_right { 1.0 } else { -1.0 } }
    pub fn just_turned(&self) -> bool { self.is_facing_right != self.was_facing_right }

    pub fn state_as_string(&self) -> String {
        match self.state {
            FighterState::Idle => String::from("Idle"),
            FighterState::Turn => String::from("Turn"),
            FighterState::Walk => String::from("Walk"),
            FighterState::Dash => String::from("Dash"),
            FighterState::Run => String::from("Run"),
            FighterState::RunBrake => String::from("RunBrake"),
            FighterState::RunTurn => String::from("RunTurn"),
            FighterState::JumpSquat => String::from("JumpSquat"),
            FighterState::Airborne => String::from("Airborne"),
            FighterState::AirDodge => String::from("AirDodge"),
            FighterState::Land => String::from("Land"),
            FighterState::LandSpecial => String::from("LandSpecial"),
        }
    }

    pub fn land(&mut self) {
        match self.state {
            FighterState::Airborne => self.set_state(FighterState::Land),
            FighterState::AirDodge => self.set_state(FighterState::LandSpecial),
            _ => ()
        }
    }

    pub fn set_ground_line(&mut self, ground_line: Option<LineSegment2D>) {
        self.ground_line = ground_line
    }
}

// State update logic.
impl Fighter {
    pub fn update(&mut self, input: &ControllerState) {
        self.input.copy_inputs(&input);

        self.was_facing_right = self.is_facing_right;
        self.position_previous = self.position;

        // Handle state transition.
        match self.state {
            FighterState::Idle => self.state_idle_transition(),
            FighterState::Turn => self.state_turn_transition(),
            FighterState::Walk => self.state_walk_transition(),
            FighterState::Dash => self.state_dash_transition(),
            FighterState::Run => self.state_run_transition(),
            FighterState::RunBrake => self.state_run_brake_transition(),
            FighterState::RunTurn => self.state_run_turn_transition(),
            FighterState::JumpSquat => self.state_jump_squat_transition(),
            FighterState::Airborne => self.state_airborne_transition(),
            FighterState::AirDodge => self.state_air_dodge_transition(),
            FighterState::Land => self.state_land_transition(),
            FighterState::LandSpecial => self.state_land_special_transition(),
        }

        // Handle state update.
        match self.state {
            FighterState::Idle => self.state_idle_update(),
            FighterState::Turn => self.state_turn_update(),
            FighterState::Walk => self.state_walk_update(),
            FighterState::Dash => self.state_dash_update(),
            FighterState::Run => self.state_run_update(),
            FighterState::RunBrake => self.state_run_brake_update(),
            FighterState::RunTurn => self.state_run_turn_update(),
            FighterState::JumpSquat => self.state_jump_squat_update(),
            FighterState::Airborne => self.state_airborne_update(),
            FighterState::AirDodge => self.state_air_dodge_update(),
            FighterState::Land => self.state_land_update(),
            FighterState::LandSpecial => self.state_land_special_update(),
        }

        //self.x_velocity = self.input.left_stick.x_axis.value();
        //self.y_velocity = self.input.left_stick.y_axis.value();
        //self.move_with_velocity();

        self.state_frame += 1;
        self.input.update();
    }

    fn handle_horizontal_air_movement(&mut self) {
        if !self.x_axis().is_active() {
            self.apply_movement_friction(self.air_friction);
        }
        else {
            self.apply_movement_acceleration(
                self.air_base_acceleration,
                self.air_axis_acceleration,
                self.air_max_velocity,
                self.air_friction,
            );
        }
    }

    fn handle_fast_fall(&mut self) {
        if self.y_velocity() <= 0.0 && self.y_axis().value() < 0.0 && self.y_axis_smashed() {
            self.set_y_velocity(-self.fast_fall_velocity);
        }
    }

    fn handle_gravity(&mut self) {
        self.set_y_velocity(self.y_velocity() - self.gravity.min(self.fall_velocity + self.y_velocity()).max(0.0));
    }

    fn move_with_velocity(&mut self) {
        self.set_x(self.x() + self.x_velocity());
        self.set_y(self.y() + self.y_velocity());
    }

    fn apply_movement_friction(&mut self, friction: f64) {
        self.set_x_velocity(self.apply_friction_to_value(self.x_velocity(), friction));
    }

    fn apply_friction_to_value(&self, velocity: f64, friction: f64) -> f64 {
        velocity - velocity.signum() * velocity.abs().min(friction)
    }

    fn apply_movement_acceleration(
        &mut self,
        base_acceleration: f64,
        axis_acceleration: f64,
        max_velocity: f64,
        friction: f64,
    ) {
        self.set_x_velocity(
            self.apply_acceleration_to_value(
                self.x_velocity(),
                self.x_axis(),
                base_acceleration,
                axis_acceleration,
                max_velocity,
                friction,
            )
        );
    }

    fn apply_acceleration_to_value(
        &self,
        velocity: f64,
        axis: &AnalogAxis,
        base_acceleration: f64,
        axis_acceleration: f64,
        max_velocity: f64,
        friction: f64
    ) -> f64 {
        let mut new_velocity = velocity;

        if velocity.abs() > max_velocity {
            new_velocity = self.apply_friction_to_value(velocity, friction);
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

// ============ STATES ============

#[derive(Copy, Clone)]
pub enum FighterState {
    Idle,
    Turn,
    Walk,
    Dash,
    Run,
    RunBrake,
    RunTurn,
    JumpSquat,
    Airborne,
    AirDodge,
    Land,
    LandSpecial,
}

// Idle.
impl Fighter {
    fn state_idle_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed() {
            self.set_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && !self.x_axis_smashed() {
            self.set_state(FighterState::Walk);
        }
        else if self.x_axis_is_backward() {
            self.set_state(FighterState::Turn);
        }
    }

    fn state_idle_update(&mut self) {
        self.apply_movement_friction(self.ground_friction);
        self.move_with_velocity();
    }
}

// Turn.
impl Fighter {
    fn state_turn_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_backward()
             && self.x_axis_smashed() {
            self.set_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.turn_frames {
            self.set_state(FighterState::Walk);
        }
        else if self.state_frame >= self.turn_frames {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_turn_update(&mut self) {
        if self.state_frame == 0 {
            match self.state_previous {
                FighterState::Dash => {
                    // I'm unsure where the 1.73 here comes from but it is necessary for now.
                    self.set_x_velocity(self.x_velocity() - self.x_velocity().signum() * 1.73);
                },
                _ => ()
            }
        }

        // Not quite right. Turn friction in melee applies on the first frame while walking,
        // but not during the one frame of turn seen while dash dancing. I need to implement that.
        if self.state_frame > 1 {
            self.apply_movement_friction(2.0 * self.ground_friction);
        }
        if self.x_axis_is_backward() && self.state_frame == self.slow_dash_back_frames {
            self.is_facing_right = self.x_axis().value() >= 0.0;
        }
        self.move_with_velocity();
    }
}

// Walk.
impl Fighter {
    fn state_walk_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed() {
            self.set_state(FighterState::Dash);
        }
        else if !self.x_axis_is_forward() {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_walk_update(&mut self) {
        if self.state_frame == 0 {
            if self.x_axis().is_active() {
                self.set_x_velocity(self.x_velocity() + self.facing_direction() * (0.1 + 0.2 * self.x_axis().value()));
            }
        }

        let target_velocity = self.walk_max_velocity * self.x_axis().value();

        if self.x_velocity().abs() > target_velocity.abs() {
            self.apply_movement_friction(2.0 * self.ground_friction);
        }
        else if self.x_axis().is_active() && self.state_frame >= 1 {
            // This isn't quite right but close-ish, not sure what the real acceleration calculation is.
            let acceleration = (target_velocity - self.x_velocity()) * 0.25 * self.x_axis().magnitude();

            self.set_x_velocity(self.x_velocity() + acceleration);

            let going_left_too_fast = target_velocity < 0.0 && self.x_velocity() < target_velocity;
            let going_right_too_fast = target_velocity > 0.0 && self.x_velocity() > target_velocity;

            if going_left_too_fast || going_right_too_fast {
                self.set_x_velocity(target_velocity);
            }
        }
        self.move_with_velocity();
    }
}

// Dash.
impl Fighter {
    fn state_dash_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.dash_max_frames {
            self.set_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.dash_min_frames
             && self.state_frame < self.dash_max_frames {
            self.set_state(FighterState::Run);
        }
        else if !self.x_axis().is_active()
             && self.state_frame >= self.dash_max_frames {
            self.set_state(FighterState::Idle);
        }
        else if self.x_axis_is_backward() {
            self.set_state(FighterState::Turn);
        }
    }

    fn state_dash_update(&mut self) {
        if self.state_frame == 0 {
            match self.state_previous {
                FighterState::Turn => { self.is_facing_right = !self.is_facing_right; },
                FighterState::Dash => { self.dash_should_reset_velocity = true; },
                _ => ()
            }
        }

        if self.state_frame == 1 {
            if self.dash_should_reset_velocity {
                self.set_x_velocity(0.0);
                self.dash_should_reset_velocity = false;
            }

            self.set_x_velocity(self.x_velocity() + self.dash_start_velocity * self.facing_direction());
            if self.x_velocity().abs() > self.dash_max_velocity {
                self.set_x_velocity(self.dash_max_velocity * self.facing_direction());
            }
        }

        if self.state_frame >= 1 {
            if !self.x_axis().is_active() {
                self.apply_movement_friction(self.ground_friction);
            }
            else {
                self.apply_movement_acceleration(
                    self.dash_base_acceleration,
                    self.dash_axis_acceleration,
                    self.dash_max_velocity,
                    self.ground_friction,
                );
            }
        }
        self.move_with_velocity();
    }
}

// Run.
impl Fighter {
    fn state_run_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if !self.x_axis().is_active() {
            self.set_state(FighterState::RunBrake);
        }
        else if self.x_axis_is_backward() {
            self.set_state(FighterState::RunTurn);
        }
    }

    fn state_run_update(&mut self) {
        let run_acceleration = ((self.dash_max_velocity * self.x_axis().value()) - self.x_velocity())
                             * (1.0 / (2.5 * self.dash_max_velocity))
                             * (self.dash_axis_acceleration + (self.dash_base_acceleration / self.x_axis().magnitude()));
        self.set_x_velocity(self.x_velocity() + run_acceleration);
        self.move_with_velocity();
    }
}

// RunBrake.
impl Fighter {
    fn state_run_brake_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_backward()
             && self.state_frame >= self.run_brake_frames {
            self.set_state(FighterState::Turn);
        }
        else if self.x_axis_is_backward()
             && self.state_frame < self.run_brake_frames {
            self.set_state(FighterState::RunTurn);
        }
        else if !self.x_axis().is_active()
             && self.state_frame >= self.run_brake_frames {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_run_brake_update(&mut self) {
        self.apply_movement_friction(self.ground_friction);
        self.move_with_velocity();
    }
}

// RunTurn.
impl Fighter {
    fn state_run_turn_transition(&mut self) {
        if self.should_jump() {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.run_turn_melee_frame >= 20 {
            self.set_state(FighterState::Run);
        }
        else if self.x_axis_is_backward()
             && self.run_turn_melee_frame >= 20 {
            self.set_state(FighterState::Turn);
        }
        else if !self.x_axis().is_active()
             && self.run_turn_melee_frame >= 20 {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_run_turn_update(&mut self) {
        if self.state_frame == 0 {
            self.run_turn_melee_frame = 0;
            self.run_turn_was_facing_right_initially = self.is_facing_right;
            self.run_turn_has_changed_direction = false;
            self.run_turn_has_fully_turned = false;
        }

        if self.run_turn_was_facing_right_initially && self.x_velocity() <= 0.0
        || !self.run_turn_was_facing_right_initially && self.x_velocity() >= 0.0 {
            self.run_turn_has_fully_turned = true;
        }

        if !self.run_turn_has_changed_direction && self.run_turn_has_fully_turned {
            self.is_facing_right = !self.is_facing_right;
            self.run_turn_has_changed_direction = true;
        }

        if !self.x_axis().is_active()
        || (!self.run_turn_has_fully_turned && self.x_axis_is_forward())
        || (self.run_turn_has_fully_turned && self.x_axis_is_backward()) {
            self.apply_movement_friction(self.ground_friction);
        }
        else {
            self.apply_movement_acceleration(
                self.dash_base_acceleration,
                self.dash_axis_acceleration,
                self.dash_max_velocity,
                self.ground_friction,
            );
        }

        self.move_with_velocity();

        if self.run_turn_has_fully_turned || (self.run_turn_melee_frame < 9 && !self.run_turn_has_fully_turned) {
            self.run_turn_melee_frame += 1;
        }
    }
}

// JumpSquat.
impl Fighter {
    fn state_jump_squat_transition(&mut self) {
        if (self.l_button().is_pressed() || self.r_button().is_pressed())
        && self.state_frame >= self.jump_squat_frames {
            self.set_state(FighterState::AirDodge);
        }
        else if self.state_frame >= self.jump_squat_frames {
            self.set_state(FighterState::Airborne);
        }
    }

    fn state_jump_squat_update(&mut self) {
        self.apply_movement_friction(2.0 * self.ground_friction);
        self.move_with_velocity();
    }
}

// Airborne.
impl Fighter {
    fn state_airborne_transition(&mut self) {
        if self.l_button().just_pressed() || self.r_button().just_pressed() {
            self.set_state(FighterState::AirDodge);
        }
    }

    fn state_airborne_update(&mut self) {
        if self.state_frame == 0 {
            match self.state_previous {
                FighterState::JumpSquat => {
                    // Handle changing horizontal velocity when jumping off of the ground based on stick x axis.
                    self.set_x_velocity((self.x_velocity() * self.jump_velocity_dampening) + (self.x_axis().value() * self.jump_start_horizontal_velocity));
                    if self.x_velocity().abs() > self.jump_max_horizontal_velocity {
                        self.set_x_velocity(self.x_velocity().signum() * self.jump_max_horizontal_velocity);
                    }

                    // Handle short hopping and full hopping.
                    if self.jump_is_active() {
                        self.set_y_velocity(self.full_hop_velocity);
                    }
                    else {
                        self.set_y_velocity(self.short_hop_velocity);
                    }
                },
                _ => ()
            }
        }

        if self.state_frame >= 1 {
            // Handle air jumps.
            if self.should_jump() && self.air_jumps_left > 0 {
                self.set_x_velocity(self.x_axis().value() * self.air_jump_horizontal_axis_multiplier);
                self.set_y_velocity(self.full_hop_velocity * self.air_jump_velocity_multiplier);
                self.air_jumps_left -= 1;
            }
            self.handle_horizontal_air_movement();
            self.handle_fast_fall();
            self.handle_gravity();
        }
        self.move_with_velocity();
    }
}

// AirDodge.
impl Fighter {
    fn state_air_dodge_transition(&mut self) {}

    fn state_air_dodge_update(&mut self) {
        if self.state_frame == 0 {
            if self.x_axis().is_active() || self.y_axis().is_active() {
                let stick_angle = self.input.left_stick.angle();
                self.set_x_velocity(3.1 * stick_angle.cos());
                self.set_y_velocity(3.1 * stick_angle.sin());
            }
            else {
                self.set_x_velocity(0.0);
                self.set_y_velocity(0.0);
            }
        }

        if self.state_frame < 30 {
            self.set_x_velocity(self.x_velocity() * 0.9);
            self.set_y_velocity(self.y_velocity() * 0.9);
        }
        else {
            self.handle_horizontal_air_movement();
            self.handle_fast_fall();
            self.handle_gravity();
        }
        self.move_with_velocity();
    }
}

// Land.
impl Fighter {
    fn state_land_transition(&mut self) {
        if self.state_frame >= 2 {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_land_update(&mut self) {
        if self.state_frame == 0 {
            self.set_y_velocity(0.0);
            self.air_jumps_left = self.air_jumps;
        }

        self.apply_movement_friction(2.0 * self.ground_friction);
        self.move_with_velocity();
    }
}

// LandSpecial.
impl Fighter {
    fn state_land_special_transition(&mut self) {
        if self.should_jump() && self.state_frame >= 9 {
            self.set_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed()
             && self.state_frame >= 9 {
            self.set_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && !self.x_axis_smashed()
             && self.state_frame >= 9 {
            self.set_state(FighterState::Walk);
        }
        else if self.x_axis_is_backward()
             && self.state_frame >= 9 {
            self.set_state(FighterState::Turn);
        }
        else if !self.x_axis().is_active()
             && self.state_frame >= 9 {
            self.set_state(FighterState::Idle);
        }
    }

    fn state_land_special_update(&mut self) {
        if self.state_frame == 0 {
            self.set_y_velocity(0.0);
            self.air_jumps_left = self.air_jumps;
        }

        let friction_multiplier = if self.state_frame < 3 { 2.0 } else { 1.0 };
        self.apply_movement_friction(friction_multiplier * self.ground_friction);
        self.move_with_velocity();
    }
}
