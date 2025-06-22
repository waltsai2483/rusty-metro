use std::f32::consts::{PI, TAU};

use ggez::glam::Vec2;

use crate::utils::{angle_between, lerp_angle};

#[derive(Clone, Copy, PartialEq)]
pub enum VehicleState {
    Moving,
    LastPlatform(f32),
    ArrivePlatform(Vec2, f32, f32, f32, f32, bool),
    LeavePlatform(Vec2, f32, f32, f32, f32, bool),
}

pub struct Segment {
    begin_pos: Vec2,
    end_pos: Vec2,
    connecting_station: usize,
    state: VehicleState,
}

impl Segment {
    pub fn new(state: VehicleState, begin_pos: Vec2, end_pos: Vec2, connecting_station: usize) -> Self {
        Segment {
            state,
            begin_pos,
            end_pos,
            connecting_station
        }
    }

    pub fn begin_pos(&self) -> Vec2 {
        self.begin_pos
    }

    pub fn end_pos(&self) -> Vec2 {
        self.end_pos
    }

    pub fn state(&self) -> VehicleState {
        self.state
    }

    pub fn length(&self) -> f32 {
        match self.state {
            VehicleState::Moving | VehicleState::LastPlatform(_) => {
                (self.end_pos - self.begin_pos).length()
            }
            VehicleState::ArrivePlatform(_, radius, entrance_angle, center_angle, _, _) => {
                radius * angle_between(center_angle, entrance_angle)
            }
            VehicleState::LeavePlatform(_, radius, _, center_angle, exit_angle, _) => {
                radius * angle_between(exit_angle, center_angle)
            }
        }
    }

    pub fn progress(&self, distance: f32, direction: f32) -> f32 {
        if direction > 0.0 {
            distance / self.length()
        } else {
            1.0 - distance / self.length()
        }
    }

    pub fn distance_to_start(&self, distance: f32, direction: f32) -> f32 {
        if direction > 0.0 {
            distance
        } else {
            self.length() - distance
        }
    }

    pub fn distance_to_end(&self, distance: f32, direction: f32) -> f32 {
        if direction > 0.0 {
            self.length() - distance
        } else {
            distance
        }
    }

    pub fn end(&self, distance: f32, direction: f32) -> bool {
        self.progress(distance, direction) >= 1.0
    }

    pub fn calculate_position(&self, distance: f32) -> Vec2 {
        match self.state {
            VehicleState::Moving | VehicleState::LastPlatform(_) => {
                self.begin_pos
                    + (self.end_pos - self.begin_pos) * (distance / self.length()).min(1.0)
            }
            VehicleState::ArrivePlatform(center, radius, entrance_angle, center_angle, _, _) => {
                let angle = lerp_angle(
                    entrance_angle,
                    center_angle,
                    (distance / self.length()).min(1.0),
                    false,
                );
                center + Vec2::from_angle(angle) * radius
            }
            VehicleState::LeavePlatform(center, radius, _, center_angle, exit_angle, _) => {
                let angle = lerp_angle(
                    center_angle,
                    exit_angle,
                    (distance / self.length()).min(1.0),
                    false,
                );
                center + Vec2::from_angle(angle) * radius
            }
        }
    }

    pub fn calculate_rotation(&self, distance: f32) -> f32 {
        match self.state {
            VehicleState::Moving | VehicleState::LastPlatform(_) => {
                let vec = self.end_pos - self.begin_pos;
                vec.y.atan2(vec.x)
            }
            VehicleState::ArrivePlatform(_, _, entrance_angle, center_angle, _, choose_larger) => {
                self.calculate_rotation_on_platform(entrance_angle, center_angle, distance)
            }
            VehicleState::LeavePlatform(_, _, _, center_angle, exit_angle, choose_larger) => {
                self.calculate_rotation_on_platform(center_angle, exit_angle, distance)
            }
        }
    }

    fn calculate_rotation_on_platform(&self, from: f32, to: f32, distance: f32) -> f32 {
        let angle = lerp_angle(
            from,
            to,
            distance / self.length(),
            false,
        );
        if (to - from).abs() <= PI {
            if to > from {
                angle + PI / 2.0
            } else {
                angle - PI / 2.0
            }
        } else {
            if to > from {
                angle - PI / 2.0
            } else {
                angle + PI / 2.0
            }
        }
    }

    pub fn set_state(&mut self, state: VehicleState) {
        self.state = state;
    }
    
    pub fn station(&self) -> usize {
        self.connecting_station
    }
}
