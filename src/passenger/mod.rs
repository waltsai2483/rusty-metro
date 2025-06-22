use ggez::glam::Vec2;

use crate::station::types::StationShape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PassengerState {
    OnStation,
    OnVehicle,
    LeavingStation(usize, Vec2),
    LeavingVehicle(usize, Vec2)
}

#[derive(Debug, Clone, Copy)]
pub struct Passenger {
    kind: StationShape,
    state: PassengerState
}

impl Passenger {
    pub fn new(kind: StationShape) -> Self {
        Passenger { kind, state: PassengerState::OnStation }
    }

    pub fn kind(&self) -> StationShape {
        self.kind
    }

    pub fn state(&self) -> PassengerState {
        self.state
    }
    
    pub fn set_state(&mut self, state: PassengerState) {
        self.state = state;
    }
}