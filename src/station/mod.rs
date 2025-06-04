use std::f32::consts::{PI, TAU};

use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam},
};
use lerp::Lerp;
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};
use rand_distr::{Distribution, Poisson};
use types::StationShape;

use crate::{
    passenger::{Passenger, PassengerState},
    shape::{Shape, ShapeBuilder},
    vehicle::{Vehicle, metro::Metro},
};

pub mod handler;
pub mod types;

const MAX_PASSENGER_RADIUS: f32 = 10.0;

pub struct Station {
    id: u32,
    kind: StationShape,
    size: f32,
    position: Vec2,
    next_spawn_distr: Poisson<f32>,
    last_spawn_time: f32,

    passengers: Vec<Passenger>,
    capacity: usize,
    prev_passenger_rotation: Vec<f32>,
    prev_passenger_radius: Vec<f32>,
}

impl Station {
    pub fn new(
        id: u32,
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
            prev_passenger_rotation: vec![],
            prev_passenger_radius: vec![],
        }
    }

    pub fn id(&self) -> u32 {
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
        self.prev_passenger_rotation.insert(0, 0.0);
        self.prev_passenger_radius.insert(0, 0.0);
    }

    pub fn take_vehicle(&mut self, vehicle: &mut Box<dyn Vehicle>) -> Vec<Passenger> {
        if vehicle.available_spaces() == 0 || self.passengers.len() == 0 {
            return vec![];
        }
        let mut moved_passengers: Vec<Passenger> = vec![];
        for passenger in self.passengers.iter_mut() {
            if passenger.state() != PassengerState::OnStation {
                continue;
            }
            moved_passengers.push(passenger.clone());
            passenger.set_state(PassengerState::LeavingStation(vehicle.id()));
        }
        return moved_passengers;
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        station_shapes: &ShapeBuilder,
        passenger_shapes: &ShapeBuilder,
    ) {
        for (i, passenger) in self.passengers.iter().enumerate() {
            self.prev_passenger_rotation[i] = self.prev_passenger_rotation[i]
                .lerp(TAU * i as f32 / self.passengers.len() as f32, 0.2);
            if let PassengerState::LeavingStation(_) = self.passengers[i].state() {
                self.prev_passenger_radius[i] = self.prev_passenger_radius[i].lerp(0.0, 0.2);
            } else {
                self.prev_passenger_radius[i] =
                    self.prev_passenger_radius[i].lerp(self.size() + MAX_PASSENGER_RADIUS, 0.1);
            }

            let pos = self.position
                + Vec2::from_angle(self.prev_passenger_rotation[i]) * self.prev_passenger_radius[i];
            passenger_shapes
                .get_mesh(passenger.kind())
                .draw(canvas, DrawParam::default().scale([0.2, 0.2]).dest(pos));
        }
        station_shapes.get_mesh(self.kind).draw(
            canvas,
            DrawParam::default()
                .scale([self.size, self.size])
                .dest(self.position),
        );
    }

    fn update(&mut self, rng: &mut ThreadRng, available_shapes: &Vec<StationShape>, delta: f32) {
        self.last_spawn_time -= delta;
        if self.last_spawn_time <= 0.0 {
            self.spawn_passenger(available_shapes.choose(rng).unwrap().clone());
            self.last_spawn_time = self.next_spawn_distr.sample(rng);
        }
    }
}
