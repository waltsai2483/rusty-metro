use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder},
    winit::platform,
};
use segment::{Segment, VehicleState};
use stop::Stop;

use crate::{
    station::{Station, handler::StationHandler},
    utils::{
        config::{PLATFORM_LINE_WIDTH, ROUTE_LINE_WIDTH},
        lerp_angle,
    },
};

pub mod handler;
pub mod segment;
pub mod stop;

pub struct Route {
    stops: Vec<Stop>,
    path_nodes: Vec<Segment>,
    color: Color,
    is_looped: bool,
    mesh: Option<Mesh>,
    dirty: bool,
}

impl Route {
    pub fn new(stops: Vec<Stop>, is_looped: bool) -> Self {
        Route {
            stops,
            path_nodes: vec![],
            color: Color::RED,
            is_looped,
            mesh: None,
            dirty: true,
        }
    }

    pub fn stops(&self) -> &Vec<Stop> {
        &self.stops
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

    fn update_route_segments(
        &mut self,
        handler: &StationHandler,
        routes_on_station: &Vec<u32>,
    ) {
        self.path_nodes.clear();

        fn calc_radius(station: &Station, route_count: u32) -> f32 {
            station.size() + (ROUTE_LINE_WIDTH + 2.0) * (route_count as f32)
        }

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
            let curr_radius = calc_radius(curr_station, routes_on_station[curr_stop.index()]);
            let next_radius = calc_radius(next_station, routes_on_station[next_stop.index()]);
            if idx == 0 {
                first_radius = curr_radius;
            }

            let next_station_vect = next_station.position() - curr_station.position();
            let perp_direction = next_station_vect.perp().normalize();
            let platform_exit = perp_direction * curr_radius * curr_stop.side_factor();
            if idx == 0 {
                first_platform_exit = platform_exit;
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
            self.path_nodes.first_mut().unwrap().set_state(VehicleState::LastPlatform(-1.0));
            self.path_nodes.last_mut().unwrap().set_state(VehicleState::LastPlatform(1.0));
        }
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
                entrance_angle,
                center_angle,
                exit_angle,
                pick_larger_angle,
            ),
            platform_entrance,
            platform_center,
        ));
        self.path_nodes.push(Segment::new(
            VehicleState::LeavePlatform(
                position,
                radius,
                entrance_angle,
                center_angle,
                exit_angle,
                pick_larger_angle,
            ),
            platform_center,
            platform_exit,
        ));
    }

    fn draw_path_mesh(&self, ctx: &Context) -> Mesh {
        let mut mb = MeshBuilder::new();

        for node in self.path_nodes.iter() {
            match node.state() {
                VehicleState::Moving | VehicleState::LastPlatform(_) => {
                    mb.line(
                        &[node.begin_pos().to_array(), node.end_pos().to_array()],
                        ROUTE_LINE_WIDTH,
                        self.color,
                    ).expect("Error creating route mesh");
                }
                VehicleState::ArrivePlatform(
                    center,
                    radius,
                    entrance_angle,
                    center_angle,
                    exit_angle,
                    pick_larger_angle,
                )
                | VehicleState::LeavePlatform(
                    center,
                    radius,
                    entrance_angle,
                    center_angle,
                    exit_angle,
                    pick_larger_angle,
                ) => {
                    let mut points: Vec<[f32; 2]> = vec![];
                    for i in -1..=21 {
                        let angle = lerp_angle(
                            entrance_angle,
                            exit_angle,
                            (i as f32) / 21.0,
                            pick_larger_angle,
                        );
                        points.push((center + Vec2::from_angle(angle) * radius).to_array());
                    }
                    mb.line(&points, PLATFORM_LINE_WIDTH, self.color)
                        .expect("Error creating route mesh");
                }
            }
        }
        Mesh::from_data(ctx, mb.build())
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        handler: &StationHandler,
        routes_on_station: &Vec<u32>,
        delta: f32,
    ) {
        if self.dirty {
            self.update_route_segments(handler, routes_on_station);
            self.mesh = Some(self.draw_path_mesh(ctx));
            self.dirty = false;
        }
    }

    pub fn draw(
        &mut self,
        ctx: &Context,
        canvas: &mut Canvas
    ) {
        if let Some(mesh) = &self.mesh {
            canvas.draw(mesh, DrawParam::default());
        }
    }

    fn add_up_routes_on_station(&self, routes_on_station: &mut Vec<u32>) {
        for i in 0..self.stops.len() {
            routes_on_station[self.stops[i].index()] += 1;
        }
    }
}
