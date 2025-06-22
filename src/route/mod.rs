use std::{
    f32::consts::{PI, SQRT_2},
    mem::swap,
};

use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder},
};
use segment::{Segment, VehicleState};
use stop::Stop;

use crate::{
    station::{Station, handler::StationHandler},
    utils::{AngleCalc, AngleNormalizer, lerp_angle},
};

pub const ROUTE_LINE_WIDTH: f32 = 4.5;
pub const PLATFORM_LINE_WIDTH: f32 = 3.0;
pub const PLATFORM_GAP_WIDTH: f32 = 0.5;
pub const PLATFORM_SMOOTHNESS: f32 = 50.0;

pub mod handler;
pub mod segment;
pub mod stop;

pub struct Route {
    id: usize,
    stops: Vec<Stop>,
    path_nodes: Vec<Segment>,
    color: Color,
    is_looped: bool,
    mesh: Option<Mesh>,
    dirty: bool,
}

impl Route {
    pub fn new(id: usize, stops: Vec<Stop>, color: Color, is_looped: bool) -> Self {
        Route {
            id,
            stops,
            path_nodes: vec![],
            color,
            is_looped,
            mesh: None,
            dirty: true,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn is_looped(&self) -> bool {
        self.is_looped
    }

    pub fn calculate_position(&self, segment_id: usize, distance: f32) -> Vec2 {
        self.path_nodes[segment_id].calculate_position(distance)
    }

    pub fn calculate_rotation(&self, segment_id: usize, distance: f32) -> f32 {
        self.path_nodes[segment_id].calculate_rotation(distance)
    }

    pub fn get(&self, segment_id: usize) -> &Segment {
        &self.path_nodes[segment_id]
    }

    pub fn length(&self) -> usize {
        self.path_nodes.len()
    }

    fn update_route_segments(&mut self, stations: &StationHandler, routes_on_station: &Vec<u32>) {
        self.path_nodes.clear();

        let mut platform_entrance: Vec2 = Vec2::new(0.0, 0.0);
        let mut first_platform_exit: Vec2 = platform_entrance;
        let mut first_radius: f32 = 0.0;
        for idx in 0..self.stops.len() {
            let curr_stop = self.stops[idx];
            let next_stop = self.stops[(idx + 1) % self.stops.len()];
            if !self.is_looped && idx == self.stops.len() - 1 {
                break;
            }
            let curr_station = stations.get(curr_stop.index());
            let next_station = stations.get(next_stop.index());
            let curr_radius = self.calc_radius(curr_station, routes_on_station[curr_stop.index()]);
            if idx == 0 {
                first_radius = curr_radius;
            }

            let (platform_exit, turning_point, next_platform_entrance) =
                self.calculate_turning_point(stations, &curr_stop, &next_stop, routes_on_station);
            if idx == 0 {
                first_platform_exit = platform_exit;
            }

            if idx > 0 {
                self.update_platform_segments(
                    curr_station.id(),
                    next_station.position() - curr_station.position(),
                    platform_entrance,
                    platform_exit,
                    curr_station.position(),
                    curr_radius,
                );
            }

            platform_entrance = next_platform_entrance;

            self.path_nodes.push(Segment::new(
                VehicleState::Moving,
                curr_station.position() + platform_exit,
                turning_point,
                curr_station.id(),
            ));
            self.path_nodes.push(Segment::new(
                VehicleState::Moving,
                turning_point,
                next_station.position() + platform_entrance,
                next_station.id(),
            ));
        }
        if self.is_looped {
            self.update_platform_segments(
                self.stops[0].index(),
                stations.get(self.stops[1].index()).position()
                    - stations.get(self.stops[0].index()).position(),
                platform_entrance,
                first_platform_exit,
                stations.get(self.stops[0].index()).position(),
                first_radius,
            );
        } else {
            self.path_nodes
                .first_mut()
                .unwrap()
                .set_state(VehicleState::LastPlatform(-1.0));
            self.path_nodes
                .last_mut()
                .unwrap()
                .set_state(VehicleState::LastPlatform(1.0));
        }
    }

    /*
    fn update_route_segments(&mut self, handler: &StationHandler, routes_on_station: &Vec<u32>) {
        self.path_nodes.clear();

        let mut platform_entrance: Vec2 = Vec2::new(0.0, 0.0);
        let mut first_platform_exit: Vec2 = platform_entrance;
        let mut first_radius: f32 = 0.0;
        for idx in 0..self.stops.len() {
            let curr_stop = self.stops[idx];
            let next_stop = self.stops[(idx + 1) % self.stops.len()];
            if !self.is_looped && idx == self.stops.len() - 1 {
                break;
            }

            let curr_station = handler.get(curr_stop.index());
            let next_station = handler.get(next_stop.index());
            let curr_radius = self.calc_radius(curr_station, routes_on_station[curr_stop.index()]);
            let next_radius = self.calc_radius(next_station, routes_on_station[next_stop.index()]);
            if idx == 0 {
                first_radius = self.calc_radius(curr_station, routes_on_station[curr_stop.index()]);
            }

            let next_station_vect = next_station.position() - curr_station.position();
            let perp_direction = next_station_vect.perp().normalize();
            let platform_exit = perp_direction * curr_radius * curr_stop.side_factor();
            if idx == 0 {
                first_platform_exit = platform_exit;
                self.calculate_turning_point(handler, &curr_stop, &next_stop, routes_on_station);
            }

            if idx > 0 {
                self.update_platform_segments(
                    next_station_vect,
                    platform_entrance,
                    platform_exit,
                    curr_station.position(),
                    curr_radius,
                );
            }

            platform_entrance = perp_direction * next_radius * next_stop.side_factor();

            self.path_nodes.push(Segment::new(
                VehicleState::Moving,
                curr_station.position() + platform_exit,
                next_station.position() + platform_entrance,
            ));
        }
        if self.is_looped {
            self.update_platform_segments(
                handler.get(self.stops[1].index()).position()
                    - handler.get(self.stops[0].index()).position(),
                platform_entrance,
                first_platform_exit,
                handler.get(self.stops[0].index()).position(),
                first_radius,
            );
        } else {
            self.path_nodes
                .first_mut()
                .unwrap()
                .set_state(VehicleState::LastPlatform(-1.0));
            self.path_nodes
                .last_mut()
                .unwrap()
                .set_state(VehicleState::LastPlatform(1.0));
        }

        fn update_platform_segments(
        &mut self,
        next_station_vect: Vec2,
        platform_entrance: Vec2,
        platform_exit: Vec2,
        position: Vec2,
        radius: f32,
    ) {
        let entrance_angle = platform_entrance.y.atan2(platform_entrance.x);
        let exit_angle = platform_exit.y.atan2(platform_exit.x);

        let pick_larger_angle = next_station_vect.dot(platform_exit - platform_entrance) < 0.0;
        let center_angle = lerp_angle(entrance_angle, exit_angle, 0.5, pick_larger_angle);

        let platform_center = Vec2::from_angle(center_angle) * radius + position;
        self.path_nodes.push(Segment::new(
            VehicleState::ArrivePlatform(
                position,
                radius,
                entrance_angle.normalize_angle(),
                center_angle.normalize_angle(),
                exit_angle.normalize_angle(),
                pick_larger_angle,
            ),
            platform_entrance,
            platform_center,
        ));
        self.path_nodes.push(Segment::new(
            VehicleState::LeavePlatform(
                position,
                radius,
                entrance_angle.normalize_angle(),
                center_angle.normalize_angle(),
                exit_angle.normalize_angle(),
                pick_larger_angle,
            ),
            platform_center,
            platform_exit,
        ));
    }
    }*/

    fn update_platform_segments(
        &mut self,
        station_id: usize,
        next_station_vect: Vec2,
        platform_entrance: Vec2,
        platform_exit: Vec2,
        position: Vec2,
        radius: f32,
    ) {
        let entrance_angle = platform_entrance.y.atan2(platform_entrance.x);
        let exit_angle = platform_exit.y.atan2(platform_exit.x);

        let pick_larger_angle = next_station_vect.dot(platform_exit - platform_entrance) < 0.0;
        let center_angle = lerp_angle(entrance_angle, exit_angle, 0.5, pick_larger_angle);

        let platform_center = Vec2::from_angle(center_angle) * radius + position;
        self.path_nodes.push(Segment::new(
            VehicleState::ArrivePlatform(
                position,
                radius,
                entrance_angle.normalize_angle(),
                center_angle.normalize_angle(),
                exit_angle.normalize_angle(),
                pick_larger_angle,
            ),
            platform_entrance,
            platform_center,
            station_id,
        ));
        self.path_nodes.push(Segment::new(
            VehicleState::LeavePlatform(
                position,
                radius,
                entrance_angle.normalize_angle(),
                center_angle.normalize_angle(),
                exit_angle.normalize_angle(),
                pick_larger_angle,
            ),
            platform_center,
            platform_exit,
            station_id,
        ));
    }

    fn calc_radius(&self, station: &Station, route_count: u32) -> f32 {
        station.size() + (ROUTE_LINE_WIDTH + PLATFORM_GAP_WIDTH) * (route_count as f32)
    }

    fn calculate_turning_point(
        &mut self,
        stations: &StationHandler,
        curr_stop: &Stop,
        next_stop: &Stop,
        routes_on_station: &Vec<u32>,
    ) -> (Vec2, Vec2, Vec2) {
        let mut begin_stop = curr_stop.index();
        let mut end_stop: usize = next_stop.index();
        let mut curr_radius =
            self.calc_radius(stations.get(begin_stop), routes_on_station[begin_stop]);
        let mut next_radius = self.calc_radius(stations.get(end_stop), routes_on_station[end_stop]);
        let reverse_factor = if curr_radius < next_radius { -1.0 } else { 1.0 };
        if reverse_factor < 0.0 {
            swap(&mut curr_radius, &mut next_radius);
            swap(&mut begin_stop, &mut end_stop);
        }
        let curr_station = stations.get(begin_stop);
        let next_station = stations.get(end_stop);

        let station_vect = next_station.position() - curr_station.position();
        let starting_direction =
            Vec2::from_angle((station_vect.angle() / (PI / 4.0) + 0.5).floor() * (PI / 4.0));
        let platform_exit =
            curr_stop.side_factor() * reverse_factor * -starting_direction.perp() * curr_radius;
        let platform_exit_vect = curr_station.position() + platform_exit;

        let exit_to_next_station = next_station.position() - platform_exit_vect;
        let turning_angle = exit_to_next_station.angle_between(starting_direction).abs();

        let turning_point_as_center = platform_exit_vect
            + starting_direction
                * (exit_to_next_station.length() * (turning_angle.cos() - turning_angle.sin()));
        let next_platform_entrance = -(next_station.position() - turning_point_as_center)
            .normalize()
            .perp()
            * next_stop.side_factor()
            * reverse_factor
            * next_radius;
        let turning_point = turning_point_as_center
            + starting_direction
                * (starting_direction.dot(next_platform_entrance)).signum()
                * next_radius
                * SQRT_2;

        if reverse_factor < 0.0 {
            (next_platform_entrance, turning_point, platform_exit)
        } else {
            (platform_exit, turning_point, next_platform_entrance)
        }
    }

    fn draw_path_mesh(&self, ctx: &Context) -> Mesh {
        let mut mb = MeshBuilder::new();
        for node in self.path_nodes.iter() {
            self.build_segment(&mut mb, node);
        }
        Mesh::from_data(ctx, mb.build())
    }

    pub fn build_segment(&self, mb: &mut MeshBuilder, node: &Segment) {
        match node.state() {
            VehicleState::Moving => {
                mb.line(
                    &[node.begin_pos(), node.end_pos()],
                    ROUTE_LINE_WIDTH,
                    self.color,
                )
                .expect("Error creating route mesh");
                mb.circle(
                    DrawMode::fill(),
                    node.begin_pos(),
                    ROUTE_LINE_WIDTH / 2.0,
                    0.1,
                    self.color,
                )
                .expect("Error creating route mesh");
                mb.circle(
                    DrawMode::fill(),
                    node.end_pos(),
                    ROUTE_LINE_WIDTH / 2.0,
                    0.1,
                    self.color,
                )
                .expect("Error creating route mesh");
            }
            VehicleState::LastPlatform(direction) => {
                mb.line(
                    &[node.begin_pos(), node.end_pos()],
                    ROUTE_LINE_WIDTH,
                    self.color,
                )
                .expect("Error creating route mesh");

                let end_node = if direction == 1.0 {
                    node.end_pos()
                } else {
                    node.begin_pos()
                };
                let perp = (node.end_pos() - node.begin_pos()).perp().normalize();
                mb.line(
                    &[
                        end_node + perp * ROUTE_LINE_WIDTH,
                        end_node - perp * ROUTE_LINE_WIDTH,
                    ],
                    ROUTE_LINE_WIDTH + 1.0,
                    self.color,
                )
                .expect("Error creating route mesh");
            }
            VehicleState::ArrivePlatform(
                center,
                radius,
                entrance_angle,
                center_angle,
                exit_angle,
                pick_larger_angle,
            ) => {
                let mut points: Vec<[f32; 2]> = vec![];
                for i in 0..=(PLATFORM_SMOOTHNESS as i32) {
                    let angle = lerp_angle(
                        entrance_angle,
                        center_angle,
                        (i as f32) / PLATFORM_SMOOTHNESS,
                        false,
                    );
                    points.push((center + Vec2::from_angle(angle) * radius).to_array());
                }
                mb.line(&points, PLATFORM_LINE_WIDTH, self.color)
                    .expect("Error creating route mesh");
            }
            VehicleState::LeavePlatform(
                center,
                radius,
                entrance_angle,
                center_angle,
                exit_angle,
                pick_larger_angle,
            ) => {
                let mut points: Vec<[f32; 2]> = vec![];
                for i in 0..=20 {
                    let angle = lerp_angle(center_angle, exit_angle, (i as f32) / 20.0, false);
                    points.push((center + Vec2::from_angle(angle) * radius).to_array());
                }
                mb.line(&points, PLATFORM_LINE_WIDTH, self.color)
                    .expect("Error creating route mesh");
            }
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        stations: &StationHandler,
        routes_on_station: &mut Vec<u32>,
        delta: f32,
    ) {
        if self.dirty {
            self.update_route_segments(stations, routes_on_station);
            self.mesh = Some(self.draw_path_mesh(ctx));
            self.dirty = false;
        }
        for idx in 1..self.stops.len() - 1 {
            routes_on_station[self.stops[idx].index()] += 1;
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) {
        if let Some(mesh) = &self.mesh {
            canvas.draw(mesh, DrawParam::default());
        }
    }
}
