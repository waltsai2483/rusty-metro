use std::collections::HashMap;

use ggez::{graphics::{Canvas, Color}, Context};

use crate::{route::handler::RouteHandler, shape::{palette::ShapePalette, ShapeBuilder}, station::handler::StationHandler};

use super::Vehicle;

pub struct VehicleHandler {
    metros: Vec<Box<dyn Vehicle>>,
    route_map: HashMap<usize, Vec<usize>>,
    max_count: u32,
    shapes: ShapeBuilder,
}

impl VehicleHandler {
    pub fn new(max_count: u32, shapes: ShapeBuilder) -> Self {
        VehicleHandler {
            metros: vec![],
            route_map: HashMap::new(),
            max_count,
            shapes
        }
    }

    pub fn vehicles(&self) -> &Vec<Box<dyn Vehicle>> {
        &self.metros
    }

    pub fn get(&self, index: usize) -> &Box<dyn Vehicle> {
        self.metros.get(index).unwrap()
    }

    pub fn metros_on_route(&self, route: usize) -> Vec<usize> {
        if self.route_map.contains_key(&route) {
            return self.route_map.get(&route).unwrap().to_vec();
        }
        return vec![];
    }

    pub fn add_vehicle(&mut self, mut vehicle: Box<dyn Vehicle>) {
        if self.max_count == self.metros.len() as u32 {
            return;
        }
        let route = vehicle.route();
        vehicle.set_id(self.metros.len());
        self.metros.push(vehicle);

        if !self.route_map.contains_key(&route) {
            self.route_map.insert(route, vec![]);
        }
        self.route_map
            .get_mut(&route)
            .unwrap()
            .push(self.metros.len() - 1);
    }

    pub fn update(&mut self, delta: f32, routes: &RouteHandler, stations: &StationHandler) {
        for vehicle in self.metros.iter_mut() {
            vehicle.update(routes, stations, delta);
        }
    }
    
    pub fn shapes(&self) -> &ShapeBuilder {
        &self.shapes
    }
}
