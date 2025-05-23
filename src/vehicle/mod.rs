use ggez::{Context, glam::Vec2, graphics::Canvas};

use crate::{
    passenger::Passenger,
    route::{handler::RouteHandler, segment::VehicleState, Route},
    shape::ShapeBuilder,
    station::{handler::StationHandler, types::StationKind},
};

pub mod handler;
pub mod metro;

pub trait Vehicle {
    fn take_vehicle(&mut self, passenger: Passenger);
    fn leave_vehicle(&mut self, kind: StationKind);

    fn set_position(&mut self, position: Vec2);
    fn set_rotation(&mut self, rotation: f32);

    fn speed(&self) -> f32;

    fn direction(&self) -> f32;
    fn reverse_direction(&mut self);

    fn update(&mut self, routes: &RouteHandler, stations: &StationHandler, delta: f32);
    fn draw(&self, canvas: &mut Canvas, shapes: &ShapeBuilder);

    fn passengers(&self) -> &Vec<Passenger>;

    fn route(&self) -> usize;

    fn segment(&self) -> usize;
    fn set_segment(&mut self, segment: usize);

    fn distance(&self) -> f32;
    fn set_distance(&mut self, distance: f32);

    fn start_next_segment(&mut self, routes: &RouteHandler) {
        let route = routes.get(self.route());
        if self.direction() < 0.0 {
            if self.segment() == 0 && route.is_looped() {
                self.set_segment(route.length() - 1);
            } else {
                self.set_segment(self.segment() - 1);
            }
        }
        self.set_distance(self.distance() - route.get(self.segment()).length() * self.direction());
        if self.direction() > 0.0 {
            self.set_segment((self.segment() + 1) % route.length());
        }
    }

    fn try_reverse_direction_at_end(&mut self, routes: &RouteHandler) -> bool {
        let route = routes.get(self.route());
        if let VehicleState::LastPlatform(direction) = route.get(self.segment()).state() {
            if direction == self.direction() {
                self.reverse_direction();
                return true;
            }
        }
        false
    }

    fn move_vehicle(&mut self, routes: &RouteHandler, stations: &StationHandler, delta: f32) {
        let route = routes.get(self.route());
        let segment = route.get(self.segment());
        self.set_position(route.calculate_position(self.segment(), self.distance()));
        while segment.end(self.distance(), self.direction()) {
            if self.try_reverse_direction_at_end(routes) {
                
            } else {
                self.start_next_segment(routes);
            }
            self.set_position(route.calculate_position(self.segment(), self.distance()));
        }
        self.set_rotation(route.calculate_rotation(self.segment(), self.distance()));
        self.set_distance(self.distance() + delta * self.speed() * self.direction());
    }
}
/*
   fn update(&mut self, routes: &RouteHandler, stations: &StationHandler, delta: f32) {
       let route = routes.get(self.route);
       self.position = route.calculate_position(self.segment, self.distance);
       while route.is_segment_end(self.segment, self.distance, self.direction) {
           if !route.is_looped() && self.segment == route.len() - 1 && self.direction == 1.0 {
               self.distance = route.get(self.segment) -.length() 0.01;
               self.direction *= -1.0;
           } else if !route.is_looped() && self.segment == 0 && self.direction == -1.0 {
               self.distance = 0.01;
               self.direction *= -1.0;
           } else {
               if self.direction < 0.0 {
                   if self.segment == 0 {
                       self.segment = route.len() - 1;
                   } else {
                       self.segment -= 1;
                   }
               }
               self.distance -= route.get(self.segment) *.length() self.direction;
               if self.direction > 0.0 {
                   self.segment = (self.segment + 1) % route.len();
               }
           }
           self.position = route.calculate_position(self.segment, self.distance);
       }
       self.rotation = route.calculate_rotation(self.segment, self.distance);
       self.distance += delta * self.speed * self.direction;
   }
*/
