use ggez::graphics::Canvas;

use crate::{route::handler::RouteHandler, shape::ShapeBuilder, station::handler::StationHandler};

use super::Vehicle;

pub struct VehicleHandler {
    metros: Vec<Box<dyn Vehicle>>,
    max_count: u32,
}

impl VehicleHandler {
    pub fn new() -> Self {
        VehicleHandler {
            metros: vec![],
            max_count: 1,
        }
    }

    pub fn vehicles(&self) -> &Vec<Box<dyn Vehicle>> {
        &self.metros
    }

    pub fn add_vehicle(&mut self, vehicle: Box<dyn Vehicle>) {
        if self.max_count == self.metros.len() as u32 {
            return;
        }
        self.metros.push(vehicle);
    }

    pub fn update(&mut self, delta: f32, routes: &RouteHandler, stations: &StationHandler) {
        for vehicle in self.metros.iter_mut() {
            vehicle.update(routes, stations, delta);
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, shapes: &ShapeBuilder) {
        for vehicle in self.metros.iter_mut() {
            vehicle.draw(canvas, shapes);
        }
    }
}
