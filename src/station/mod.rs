use std::f32::consts::{PI, TAU};

use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam},
};
use lerp::Lerp;
use rand::{Rng, rngs::StdRng, seq::IndexedRandom};
use rand_distr::{Distribution, Poisson};
use types::StationShape;

use crate::{
    passenger::{Passenger, PassengerState},
    shape::{Shape, ShapeBuilder},
    vehicle::{Vehicle, handler::VehicleHandler, metro::Metro},
};

pub mod handler;
pub mod types;

const MAX_PASSENGER_RADIUS: f32 = 10.0;

pub struct PassengerOnStation {
    passenger: Passenger,
    prev_passenger_rotation: f32,
    prev_passenger_radius: f32,
    prev_passenger_position: Vec2,
}

pub struct Station {
    id: usize,
    kind: StationShape,
    size: f32,
    position: Vec2,
    next_spawn_distr: Poisson<f32>,
    last_spawn_time: f32,

    passengers: Vec<Passenger>,
    passenger_render_state: Vec<(f32, f32, Vec2)>,
    capacity: usize,
}

impl Station {
    pub fn new(
        id: usize,
        kind: StationShape,
        size: f32,
        position: Vec2,
        passenger_spawn_rate: f32,
        capacity: usize,
    ) -> Self {
        assert!(passenger_spawn_rate > 0.0 && size > 0.0);
        Station {
            id,
            kind,
            size,
            position,
            next_spawn_distr: Poisson::new(passenger_spawn_rate).unwrap(),
            passengers: vec![],
            last_spawn_time: 0.0,
            capacity,
            passenger_render_state: vec![],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn kind(&self) -> StationShape {
        self.kind
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn size(&self) -> f32 {
        self.size * 15.0
    }

    pub fn spawn_passenger(&mut self, kind: StationShape) {
        self.passengers.push(Passenger::new(kind));
        self.passenger_render_state.push((0.0, 0.0, self.position));
    }

    pub fn try_take_vehicle(&mut self, vehicle: &mut dyn Vehicle) -> Vec<Passenger> {
        if vehicle.available_spaces() == 0 || self.passengers.len() == 0 {
            return vec![];
        }
        let mut moved_passengers: Vec<Passenger> = vec![];
        for (i, passenger) in self.passengers.iter_mut().enumerate() {
            if passenger.state() != PassengerState::OnStation {
                continue;
            }
            moved_passengers.push(passenger.clone());
            self.passenger_render_state[i] = (vehicle.position().distance(self.passenger_render_state[i].2), 0.025, self.passenger_render_state[i].2);
            passenger.set_state(PassengerState::LeavingStation(
                vehicle.id(),
                vehicle.position(),
            ));
        }
        return moved_passengers;
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        vehicles: &VehicleHandler,
        station_shapes: &ShapeBuilder,
        passenger_shapes: &ShapeBuilder,
    ) {
        for (i, passenger) in self.passengers.iter().enumerate() {
            if !matches!(
                self.passengers[i].state(),
                PassengerState::LeavingStation(..)
            ) {
                self.passenger_render_state[i].0 = self.passenger_render_state[i].0
                    .lerp(TAU * i as f32 / self.passengers.len() as f32, 0.05);
                self.passenger_render_state[i].1 =
                    self.passenger_render_state[i].1.lerp(self.size() + MAX_PASSENGER_RADIUS, 0.1);
                self.passenger_render_state[i].2 = self.position
                    + Vec2::from_angle(self.passenger_render_state[i].0)
                        * self.passenger_render_state[i].1;
                passenger_shapes.get_mesh(passenger.kind()).draw(
                    canvas,
                    DrawParam::default()
                        .scale([0.2, 0.2])
                        .dest(self.passenger_render_state[i].2),
                );
            }
        }
        station_shapes.get_mesh(self.kind).draw(
            canvas,
            DrawParam::default()
                .scale([self.size, self.size])
                .dest(self.position),
        );
        for (i, passenger) in self.passengers.iter().enumerate() {
            if let PassengerState::LeavingStation(_, pos) = self.passengers[i].state() {
                self.passenger_render_state[i].2 = self.passenger_render_state[i].2.lerp(pos, 0.07);
                
                let scale = ((pos.distance(self.passenger_render_state[i].2) - 0.05) / self.passenger_render_state[i].0) * 0.2 + 0.05;
                passenger_shapes.get_mesh(passenger.kind()).draw(
                    canvas,
                    DrawParam::default()
                        .scale([scale, scale])
                        .dest(self.passenger_render_state[i].2),
                );
            }
        }
    }

    fn update(&mut self, rng: &mut StdRng, available_shapes: &Vec<StationShape>, delta: f32) {
        self.last_spawn_time -= delta;
        if self.last_spawn_time <= 0.0 {
            self.spawn_passenger(available_shapes.choose(rng).unwrap().clone());
            self.last_spawn_time = self.next_spawn_distr.sample(rng);
        }
        if !self.passengers.is_empty() {
            for i in (0..self.passengers.len()).rev() {
                if let PassengerState::LeavingStation(_, pos) = self.passengers[i].state() {
                    if pos.distance(self.passenger_render_state[i].2) < 0.05 {
                        self.passengers.remove(i);
                        self.passenger_render_state.remove(i);
                    }
                }
            }
        }
    }
}
