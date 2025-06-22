
use ggez::{
    glam::Vec2,
    graphics::Canvas,
};
use rand::rngs::StdRng;

use crate::{shape::{palette::ShapePalette, Shape, ShapeBuilder}, vehicle::handler::VehicleHandler};

use super::{Station, types::StationShape};

pub struct StationHandler {
    station_shapes: ShapeBuilder,
    passenger_shapes: ShapeBuilder,
    available_shapes: Vec<StationShape>,
    stations: Vec<Station>
}

impl StationHandler {
    pub fn new(station_shapes: ShapeBuilder, passenger_shapes: ShapeBuilder) -> Self {
        StationHandler {
            station_shapes,
            passenger_shapes,
            available_shapes: vec![],
            stations: vec![]
        }
    }

    pub fn add_station(&mut self, kind: StationShape, position: Vec2) {
        self.stations
            .push(Station::new(self.stations.len(), kind, 1.0, position, 5.0, 10));
        if !self.available_shapes.contains(&kind) {
            self.available_shapes.push(kind);
        }
    }

    pub fn stations(&self) -> &Vec<Station> {
        &self.stations
    }

    pub fn draw(&mut self, canvas: &mut Canvas, vehicles: &VehicleHandler) {
        for station in self.stations.iter_mut() {
            station.draw(canvas, vehicles, &self.station_shapes, &self.passenger_shapes);
        }
    }

    pub fn update(&mut self, rng: &mut StdRng, delta: f32) {
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
