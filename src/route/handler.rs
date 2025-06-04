use ggez::{
    Context,
    graphics::{Canvas, Color},
};

use crate::{station::handler::StationHandler, utils::colors::Colors};

use super::{Route, stop::Stop};

pub struct RouteHandler {
    routes: Vec<Route>,
    palette: Vec<Color>,
}

impl RouteHandler {
    pub fn new() -> Self {
        RouteHandler {
            routes: vec![],
            palette: Colors::default_palette()
        }
    }

    pub fn get(&self, index: usize) -> &Route {
        &self.routes[index]
    }

    pub fn add_route(&mut self, stops: Vec<Stop>, is_looped: bool) {
        self.routes.push(Route::new(
            self.routes.len(),
            stops,
            self.palette[self.routes.len()],
            is_looped,
        ));
    }

    pub fn update(&mut self, ctx: &Context, stations: &StationHandler, delta: f32) {
        let mut routes_on_station = vec![1; stations.stations().len()];
        for route in self.routes.iter_mut() {
            route.update(ctx, &stations, &mut routes_on_station, delta);
        }
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, Route> {
        self.routes.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, Route> {
        self.routes.iter_mut()
    }
}
