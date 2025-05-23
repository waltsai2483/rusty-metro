use ggez::{graphics::Canvas, Context};

use crate::station::handler::StationHandler;

use super::Route;

pub struct RouteHandler {
    routes: Vec<Route>
}

impl RouteHandler {
    pub fn new(routes: Vec<Route>) -> Self {
        RouteHandler { routes }
    }

    pub fn get(&self, index: usize) -> &Route { &self.routes[index] }

    pub fn update(&mut self, ctx: &Context, handler: &StationHandler, delta: f32) {
        let mut routes_on_station = vec![1; handler.stations().len()];
        for route in self.routes.iter_mut() {
            route.update(ctx, &handler, &mut routes_on_station, delta);
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas, handler: &StationHandler) {
        let mut routes_on_station = vec![1; handler.stations().len()];
        for route in self.routes.iter_mut() {
            route.draw(ctx, canvas);
            route.add_up_routes_on_station(&mut routes_on_station);
        }
    }
}