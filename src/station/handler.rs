use std::ops::Index;

use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, DrawParam},
};
use rand::rngs::ThreadRng;

use crate::shape::{Shape, ShapeBuilder, palette::ShapePalette};

use super::{Station, types::StationShape};

pub struct StationHandler {
    station_shapes: ShapeBuilder,
    passenger_shapes: ShapeBuilder,
    available_shapes: Vec<StationShape>,
    stations: Vec<Station>,
    counter: u32,
}

impl StationHandler {
    pub fn new(station_shapes: ShapeBuilder, passenger_shapes: ShapeBuilder) -> Self {
        StationHandler {
            station_shapes,
            passenger_shapes,
            available_shapes: vec![],
            stations: vec![],
            counter: 0,
        }
    }

    pub fn add_station(&mut self, kind: StationShape, position: Vec2) {
        self.stations
            .push(Station::new(self.counter, kind, 1.0, position, 5.0, 10));
        self.counter += 1;
        if !self.available_shapes.contains(&kind) {
            self.available_shapes.push(kind);
        }
    }

    pub fn stations(&self) -> &Vec<Station> {
        &self.stations
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        for station in self.stations.iter_mut() {
            station.draw(canvas, &self.station_shapes, &self.passenger_shapes);
        }
    }

    pub fn update(&mut self, rng: &mut ThreadRng, delta: f32) {
        for station in self.stations.iter_mut() {
            station.update(rng, &self.available_shapes, delta);
        }
    }

    pub fn get(&self, index: usize) -> &Station {
        &self.stations[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Station {
        &mut self.stations[index]
    }
}
