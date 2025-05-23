use std::ops::Index;

use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, DrawParam},
};

use crate::shape::{Shape, ShapeBuilder, palette::ShapePalette};

use super::{Station, types::StationKind};

pub struct StationHandler {
    shape_builder: ShapeBuilder,
    stations: Vec<Station>,
    counter: u32,
}

impl StationHandler {
    pub fn new(ctx: &mut Context, shape_color: ShapePalette) -> Self {
        StationHandler {
            shape_builder: ShapeBuilder::new(ctx, shape_color),
            stations: vec![],
            counter: 0
        }
    }

    pub fn add_station(&mut self, kind: StationKind, position: Vec2) {
        self.stations.push(Station::new(self.counter, kind, 1.0, position));
        self.counter += 1;
    }

    pub fn shapes(&self) -> &ShapeBuilder {
        &self.shape_builder
    }

    pub fn stations(&self) -> &Vec<Station> {
        &self.stations
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for station in self.stations.iter() {
            station.draw(canvas, self.shapes());
        }
    }

    pub fn get(&self, index: usize) -> &Station {
        &self.stations[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Station {
        &mut self.stations[index]
    }
}
