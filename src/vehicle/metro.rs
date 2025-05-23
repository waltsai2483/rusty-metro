use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Quad, Rect},
};

use crate::{
    passenger::Passenger,
    route::{handler::RouteHandler, segment::VehicleState},
    shape::ShapeBuilder,
    station::handler::StationHandler,
};

use super::Vehicle;

pub struct Metro {
    color: Color,
    route: usize,
    segment: usize,
    distance: f32,

    stopping: bool,
    speed: f32,
    max_speed: f32,
    waiting_time: f32,
    max_waiting_time: f32,

    direction: f32,

    position: Vec2,
    rotation: f32,
    mesh: Mesh,

    passengers: Vec<Passenger>,
}

impl Metro {
    pub fn new(ctx: &Context, route: usize, color: Color) -> Self {
        Metro {
            color,
            route,
            segment: 0,
            distance: 0.0,
            stopping: true,
            speed: 0.0,
            max_speed: 200.0,
            direction: 1.0,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            passengers: vec![],
            waiting_time: 0.0,
            max_waiting_time: 1.0,
            mesh: Mesh::from_data(
                ctx,
                MeshBuilder::new()
                    .rectangle(DrawMode::fill(), Rect::new(-0.5, -0.5, 1.0, 1.0), color)
                    .unwrap()
                    .triangles(&[[0.5, -0.5], [0.5, 0.5], [1.0, 0.0]], color)
                    .unwrap()
                    .build(),
            ),
        }
    }

    fn is_vehicle_stopping(
        &mut self,
        routes: &RouteHandler,
        stations: &StationHandler,
        delta: f32,
    ) -> bool {
        let segment = routes.get(self.route).get(self.segment);

        if self.stopping {
            self.waiting_time += delta;
            if self.waiting_time > self.max_waiting_time {
                self.stopping = false;
                if !self.try_reverse_direction_at_end(routes) {
                    self.start_next_segment(routes);
                    self.distance = segment.progress(0.0, self.direction) * segment.length(); // Move to start of the LeavePlatform = platform
                    self.speed = 0.0;
                }
            } else {
                self.distance = (1.0 - segment.progress(1.0, self.direction)) * segment.length(); // Stay at end of the ArrivePlatform = platform
            }
        }

        if !self.stopping {
            match segment.state() {
                VehicleState::Moving => {
                    self.speed += (self.max_speed - self.speed) * 0.25 * delta;
                }
                VehicleState::LastPlatform(arrive_direction) => {
                    if self.direction == arrive_direction {
                        self.speed = self.max_speed
                            * (0.05
                                + segment.distance_to_end(self.distance, self.direction) / 50.0)
                                .min(1.0);
                        if segment.end(self.distance, self.direction) {
                            self.stopping = true;
                            self.waiting_time = 0.0;
                        }
                    } else {
                        self.speed = self.max_speed
                            * (segment.distance_to_start(self.distance, self.direction) / 50.0)
                                .min(1.0);
                    }
                }
                VehicleState::ArrivePlatform(..) => {
                    if self.direction > 0.0 {
                        self.speed = self.max_speed
                            * (1.05 - segment.progress(self.distance, self.direction));
                        if segment.end(self.distance, self.direction) {
                            self.stopping = true;
                            self.waiting_time = 0.0;
                        }
                    } else {
                        self.speed =
                            self.max_speed * segment.progress(self.distance, self.direction);
                    }
                }
                VehicleState::LeavePlatform(..) => {
                    if self.direction < 0.0 {
                        self.speed = self.max_speed
                            * (1.05 - segment.progress(self.distance, self.direction));
                        if segment.end(self.distance, self.direction) {
                            self.stopping = true;
                            self.waiting_time = 0.0;
                        }
                    } else {
                        self.speed =
                            self.max_speed * segment.progress(self.distance, self.direction);
                    }
                }
            }
        }
        self.stopping
    }
}

impl Vehicle for Metro {
    fn take_vehicle(&mut self, passenger: crate::passenger::Passenger) {
        todo!()
    }

    fn leave_vehicle(&mut self, kind: crate::station::types::StationKind) {
        todo!()
    }

    fn draw(&self, canvas: &mut Canvas, shapes: &ShapeBuilder) {
        canvas.draw(
            &self.mesh,
            DrawParam::default()
                .dest(self.position)
                .rotation(self.rotation)
                .scale([20.0, 10.0])
                .color(self.color),
        );
    }

    fn update(&mut self, routes: &RouteHandler, stations: &StationHandler, delta: f32) {
        if self.is_vehicle_stopping(routes, stations, delta) {
            return;
        }
        self.move_vehicle(routes, stations, delta);
    }

    fn passengers(&self) -> &Vec<Passenger> {
        &self.passengers
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn direction(&self) -> f32 {
        self.direction
    }

    fn reverse_direction(&mut self) {
        self.direction *= -1.0;
    }

    fn route(&self) -> usize {
        self.route
    }

    fn segment(&self) -> usize {
        self.segment
    }

    fn set_segment(&mut self, segment: usize) {
        self.segment = segment;
    }

    fn distance(&self) -> f32 {
        self.distance
    }

    fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }
}
