use crate::controller_state::ControllerState;
use crate::analog_axis::AnalogAxis;
use crate::point_math::Point;
use crate::vector_math::Vector;

pub struct Fighter {
    pub input: ControllerState,
    pub position: Point,
    pub previous_position: Point,
    pub velocity: Vector,
    pub air_jumps_left: u32,
    pub is_facing_right: bool,
    pub was_facing_right: bool,
    pub state: FighterState,
    pub previous_state: FighterState,
    pub state_frame: u32,
    pub dash_should_reset_velocity: bool,
    pub run_turn_melee_frame: u32,
    pub run_turn_was_facing_right_initially: bool,
    pub run_turn_has_changed_direction: bool,
    pub run_turn_has_fully_turned: bool,

    pub ground_friction: f64,
    pub dash_start_velocity: f64,
    pub dash_max_velocity: f64,
    pub dash_base_acceleration: f64,
    pub dash_axis_acceleration: f64,
    pub walk_start_velocity: f64,
    pub walk_max_velocity: f64,
    pub walk_acceleration: f64,
    pub air_friction: f64,
    pub air_base_acceleration: f64,
    pub air_axis_acceleration: f64,
    pub air_max_velocity: f64,
    pub jump_squat_frames: u32,
    pub jump_velocity_dampening: f64,
    pub jump_max_horizontal_velocity: f64,
    pub jump_start_horizontal_velocity: f64,
    pub short_hop_velocity: f64,
    pub full_hop_velocity: f64,
    pub fall_velocity: f64,
    pub fast_fall_velocity: f64,
    pub air_jump_velocity_multiplier: f64,
    pub air_jump_horizontal_axis_multiplier: f64,
    pub air_jumps: u32,
    pub gravity: f64,
    pub dash_min_frames: u32,
    pub dash_max_frames: u32,
    pub slow_dash_back_frames: u32,
    pub turn_frames: u32,
    pub run_brake_frames: u32,
}

// Character builders.
impl Fighter {
    pub fn default() -> Fighter {
        Fighter {
            input: ControllerState::default(),
            position: Point::default(),
            previous_position: Point::default(),
            velocity: Vector::default(),
            air_jumps_left: 0,
            is_facing_right: true,
            was_facing_right: true,
            state: FighterState::Idle,
            previous_state: FighterState::Idle,
            state_frame: 0,
            dash_should_reset_velocity: false,
            run_turn_melee_frame: 0,
            run_turn_was_facing_right_initially: true,
            run_turn_has_changed_direction: false,
            run_turn_has_fully_turned: false,

            ground_friction: 0.0,
            dash_start_velocity: 0.0,
            dash_max_velocity: 0.0,
            dash_base_acceleration: 0.0,
            dash_axis_acceleration: 0.0,
            walk_start_velocity: 0.0,
            walk_max_velocity: 0.0,
            walk_acceleration: 0.0,
            air_friction: 0.0,
            air_base_acceleration: 0.0,
            air_axis_acceleration: 0.0,
            air_max_velocity: 0.0,
            jump_squat_frames: 0,
            jump_velocity_dampening: 0.0,
            jump_max_horizontal_velocity: 0.0,
            jump_start_horizontal_velocity: 0.0,
            short_hop_velocity: 0.0,
            full_hop_velocity: 0.0,
            fall_velocity: 0.0,
            fast_fall_velocity: 0.0,
            air_jump_velocity_multiplier: 0.0,
            air_jump_horizontal_axis_multiplier: 0.0,
            air_jumps: 0,
            gravity: 0.0,
            dash_min_frames: 0,
            dash_max_frames: 0,
            slow_dash_back_frames: 0,
            turn_frames: 0,
            run_brake_frames: 0,
        }
    }

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

            ..Fighter::default()
        }
    }
}

// Methods.
impl Fighter {
    pub fn change_state(&mut self, new_state: FighterState) {
        self.state_frame = 0;
        self.previous_state = self.state;
        self.state = new_state;
    }

    pub fn facing_direction(&self) -> f64 {
        if self.is_facing_right {
            1.0
        }
        else {
            -1.0
        }
    }

    pub fn just_turned(&self) -> bool {
        self.is_facing_right != self.was_facing_right
    }

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

    pub fn can_land(&self) -> bool {
        self.state == FighterState::Airborne
        || self.state == FighterState::AirDodge
    }

    pub fn land(&mut self) {
        match self.state {
            FighterState::Airborne => self.change_state(FighterState::Land),
            FighterState::AirDodge => self.change_state(FighterState::LandSpecial),
            _ => ()
        }
    }

    pub fn should_jump(&self) -> bool {
        self.input.x_button.just_pressed() || self.input.y_button.just_pressed()
    }

    pub fn jump_is_active(&self) -> bool {
        self.input.x_button.is_pressed || self.input.y_button.is_pressed
    }

    pub fn x_axis_is_forward(&self) -> bool {
        let x_axis = &self.input.x_axis;
        x_axis.is_active() && (x_axis.value > 0.0 && self.is_facing_right
                            || x_axis.value < 0.0 && !self.is_facing_right)
    }

    pub fn x_axis_is_backward(&self) -> bool {
        self.input.x_axis.is_active() && !self.x_axis_is_forward()
    }

    pub fn x_axis_smashed(&self) -> bool {
        self.input.x_axis.value.abs() >= 0.8 && self.input.x_axis.frames_active < 2
    }

    pub fn y_axis_smashed(&self) -> bool {
        self.input.y_axis.value.abs() >= 0.6625 && self.input.y_axis.frames_active < 2
    }
}

// State update logic.
impl Fighter {
    pub fn update(&mut self, input: &ControllerState) {
        self.input.update();
        self.input.copy_inputs(&input);

        self.was_facing_right = self.is_facing_right;
        self.previous_position.x = self.position.x;
        self.previous_position.y = self.position.y;

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

        //self.velocity.x = self.input.left_stick.x_axis.value;
        //self.velocity.y = self.input.left_stick.y_axis.value;
        //self.move_with_velocity();

        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.land();
        }

        self.state_frame += 1;
    }

    fn handle_horizontal_air_movement(&mut self) {
        if !self.input.x_axis.is_active() {
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
        if self.velocity.y <= 0.0 && self.input.y_axis.value < 0.0 && self.y_axis_smashed() {
            self.velocity.y = -self.fast_fall_velocity;
        }
    }

    fn handle_gravity(&mut self) {
        self.velocity.y -= self.gravity.min(self.fall_velocity + self.velocity.y).max(0.0);
    }

    fn move_with_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.x += self.velocity.y;
    }

    fn apply_movement_friction(&mut self, friction: f64) {
        self.velocity.x = self.apply_friction_to_value(self.velocity.x, friction);
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
        self.velocity.x = self.apply_acceleration_to_value(
            self.velocity.x,
            &self.input.x_axis,
            base_acceleration,
            axis_acceleration,
            max_velocity,
            friction,
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

        let mut acceleration = axis.direction() * base_acceleration + axis.value * axis_acceleration;

        if axis.value > 0.0 {
            acceleration = acceleration.min(max_velocity - new_velocity);
            acceleration = acceleration.max(0.0);
            new_velocity += acceleration;
        }
        else if axis.value < 0.0 {
            acceleration = acceleration.max(-max_velocity - new_velocity);
            acceleration = acceleration.min(0.0);
            new_velocity += acceleration;
        }

        new_velocity
    }
}

// ============ STATES ============

#[derive(Copy, Clone, PartialEq)]
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
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed() {
            self.change_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && !self.x_axis_smashed() {
            self.change_state(FighterState::Walk);
        }
        else if self.x_axis_is_backward() {
            self.change_state(FighterState::Turn);
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
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_backward()
             && self.x_axis_smashed() {
            self.change_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.turn_frames {
            self.change_state(FighterState::Walk);
        }
        else if self.state_frame >= self.turn_frames {
            self.change_state(FighterState::Idle);
        }
    }

    fn state_turn_update(&mut self) {
        if self.state_frame == 0 {
            match self.previous_state {
                FighterState::Dash => {
                    // I'm unsure where the 1.73 here comes from but it is necessary for now.
                    self.velocity.x -= self.velocity.x.signum() * 1.73;
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
            self.is_facing_right = self.input.x_axis.value >= 0.0;
        }
        self.move_with_velocity();
    }
}

// Walk.
impl Fighter {
    fn state_walk_transition(&mut self) {
        if self.should_jump() {
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed() {
            self.change_state(FighterState::Dash);
        }
        else if !self.x_axis_is_forward() {
            self.change_state(FighterState::Idle);
        }
    }

    fn state_walk_update(&mut self) {
        if self.state_frame == 0 {
            if self.input.x_axis.is_active() {
                self.velocity.x += self.facing_direction() * (0.1 + 0.2 * self.input.x_axis.value);
            }
        }

        let target_velocity = self.walk_max_velocity * self.input.x_axis.value;

        if self.velocity.x.abs() > target_velocity.abs() {
            self.apply_movement_friction(2.0 * self.ground_friction);
        }
        else if self.input.x_axis.is_active() && self.state_frame >= 1 {
            // This isn't quite right but close-ish, not sure what the real acceleration calculation is.
            let acceleration = (target_velocity - self.velocity.x) * 0.25 * self.input.x_axis.value.abs();

            self.velocity.x += acceleration;

            let going_left_too_fast = target_velocity < 0.0 && self.velocity.x < target_velocity;
            let going_right_too_fast = target_velocity > 0.0 && self.velocity.x > target_velocity;

            if going_left_too_fast || going_right_too_fast {
                self.velocity.x = target_velocity;
            }
        }
        self.move_with_velocity();
    }
}

// Dash.
impl Fighter {
    fn state_dash_transition(&mut self) {
        if self.should_jump() {
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.dash_max_frames {
            self.change_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && self.state_frame >= self.dash_min_frames
             && self.state_frame < self.dash_max_frames {
            self.change_state(FighterState::Run);
        }
        else if !self.input.x_axis.is_active()
             && self.state_frame >= self.dash_max_frames {
            self.change_state(FighterState::Idle);
        }
        else if self.x_axis_is_backward() {
            self.change_state(FighterState::Turn);
        }
    }

    fn state_dash_update(&mut self) {
        if self.state_frame == 0 {
            match self.previous_state {
                FighterState::Turn => { self.is_facing_right = !self.is_facing_right; },
                FighterState::Dash => { self.dash_should_reset_velocity = true; },
                _ => ()
            }
        }

        if self.state_frame == 1 {
            if self.dash_should_reset_velocity {
                self.velocity.x = 0.0;
                self.dash_should_reset_velocity = false;
            }

            self.velocity.x += self.dash_start_velocity * self.facing_direction();
            if self.velocity.x.abs() > self.dash_max_velocity {
                self.velocity.x = self.dash_max_velocity * self.facing_direction();
            }
        }

        if self.state_frame >= 1 {
            if !self.input.x_axis.is_active() {
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
            self.change_state(FighterState::JumpSquat);
        }
        else if !self.input.x_axis.is_active() {
            self.change_state(FighterState::RunBrake);
        }
        else if self.x_axis_is_backward() {
            self.change_state(FighterState::RunTurn);
        }
    }

    fn state_run_update(&mut self) {
        let run_acceleration = ((self.dash_max_velocity * self.input.x_axis.value) - self.velocity.x)
                             * (1.0 / (2.5 * self.dash_max_velocity))
                             * (self.dash_axis_acceleration + (self.dash_base_acceleration / self.input.x_axis.value.abs()));
        self.velocity.x += run_acceleration;
        self.move_with_velocity();
    }
}

// RunBrake.
impl Fighter {
    fn state_run_brake_transition(&mut self) {
        if self.should_jump() {
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_backward()
             && self.state_frame >= self.run_brake_frames {
            self.change_state(FighterState::Turn);
        }
        else if self.x_axis_is_backward()
             && self.state_frame < self.run_brake_frames {
            self.change_state(FighterState::RunTurn);
        }
        else if !self.input.x_axis.is_active()
             && self.state_frame >= self.run_brake_frames {
            self.change_state(FighterState::Idle);
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
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.run_turn_melee_frame >= 20 {
            self.change_state(FighterState::Run);
        }
        else if self.x_axis_is_backward()
             && self.run_turn_melee_frame >= 20 {
            self.change_state(FighterState::Turn);
        }
        else if !self.input.x_axis.is_active()
             && self.run_turn_melee_frame >= 20 {
            self.change_state(FighterState::Idle);
        }
    }

    fn state_run_turn_update(&mut self) {
        if self.state_frame == 0 {
            self.run_turn_melee_frame = 0;
            self.run_turn_was_facing_right_initially = self.is_facing_right;
            self.run_turn_has_changed_direction = false;
            self.run_turn_has_fully_turned = false;
        }

        if self.run_turn_was_facing_right_initially && self.velocity.x <= 0.0
        || !self.run_turn_was_facing_right_initially && self.velocity.x >= 0.0 {
            self.run_turn_has_fully_turned = true;
        }

        if !self.run_turn_has_changed_direction && self.run_turn_has_fully_turned {
            self.is_facing_right = !self.is_facing_right;
            self.run_turn_has_changed_direction = true;
        }

        if !self.input.x_axis.is_active()
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
        if (self.input.l_button.is_pressed || self.input.r_button.is_pressed)
        && self.state_frame >= self.jump_squat_frames {
            self.change_state(FighterState::AirDodge);
        }
        else if self.state_frame >= self.jump_squat_frames {
            self.change_state(FighterState::Airborne);
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
        if self.input.l_button.just_pressed() || self.input.r_button.just_pressed() {
            self.change_state(FighterState::AirDodge);
        }
    }

    fn state_airborne_update(&mut self) {
        if self.state_frame == 0 {
            match self.previous_state {
                FighterState::JumpSquat => {
                    // Handle changing horizontal velocity when jumping off of the ground based on stick x axis.
                    self.velocity.x = (self.velocity.x * self.jump_velocity_dampening) + (self.input.x_axis.value * self.jump_start_horizontal_velocity);
                    if self.velocity.x.abs() > self.jump_max_horizontal_velocity {
                        self.velocity.x = self.velocity.x.signum() * self.jump_max_horizontal_velocity;
                    }

                    // Handle short hopping and full hopping.
                    if self.jump_is_active() {
                        self.velocity.y = self.full_hop_velocity;
                    }
                    else {
                        self.velocity.y = self.short_hop_velocity;
                    }
                },
                _ => ()
            }
        }

        if self.state_frame >= 1 {
            // Handle air jumps.
            if self.should_jump() && self.air_jumps_left > 0 {
                self.velocity.x = self.input.x_axis.value * self.air_jump_horizontal_axis_multiplier;
                self.velocity.y = self.full_hop_velocity * self.air_jump_velocity_multiplier;
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
            if self.input.x_axis.is_active() || self.input.y_axis.is_active() {
                let stick_angle = {
                    let stick_vector = Vector {
                        x: self.input.x_axis.value,
                        y: self.input.y_axis.value,
                    };
                    stick_vector.angle()
                };
                self.velocity.x = 3.1 * stick_angle.cos();
                self.velocity.y = 3.1 * stick_angle.sin();
            }
            else {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
        }

        if self.state_frame < 30 {
            self.velocity.x += 0.9;
            self.velocity.y += 0.9;
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
            self.change_state(FighterState::Idle);
        }
    }

    fn state_land_update(&mut self) {
        if self.state_frame == 0 {
            self.velocity.y = 0.0;
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
            self.change_state(FighterState::JumpSquat);
        }
        else if self.x_axis_is_forward()
             && self.x_axis_smashed()
             && self.state_frame >= 9 {
            self.change_state(FighterState::Dash);
        }
        else if self.x_axis_is_forward()
             && !self.x_axis_smashed()
             && self.state_frame >= 9 {
            self.change_state(FighterState::Walk);
        }
        else if self.x_axis_is_backward()
             && self.state_frame >= 9 {
            self.change_state(FighterState::Turn);
        }
        else if !self.input.x_axis.is_active()
             && self.state_frame >= 9 {
            self.change_state(FighterState::Idle);
        }
    }

    fn state_land_special_update(&mut self) {
        if self.state_frame == 0 {
            self.velocity.y = 0.0;
            self.air_jumps_left = self.air_jumps;
        }

        let friction_multiplier = if self.state_frame < 3 { 2.0 } else { 1.0 };
        self.apply_movement_friction(friction_multiplier * self.ground_friction);
        self.move_with_velocity();
    }
}
