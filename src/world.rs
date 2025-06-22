use ggez::{
    Context, GameError, GameResult,
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, FilterMode, Quad, Rect},
};
use rand::{SeedableRng, rngs::StdRng};

use crate::{
    route::{
        handler::RouteHandler,
        stop::{Stop, StopSide},
    },
    shape::{ShapeBuilder, palette::ShapePalette},
    station::{handler::StationHandler, types::StationShape},
    utils::colors::Colors,
    vehicle::{handler::VehicleHandler, metro::Metro},
};

pub struct MetroWorld {
    rng: StdRng,
    time: f32,

    logical_width: f32,
    logical_height: f32,
    screen_transform_rect: Rect,

    stations: StationHandler,
    routes: RouteHandler,
    vehicles: VehicleHandler,
}

impl MetroWorld {
    pub fn new(ctx: &mut Context, seed: u64) -> Self {
        let (logical_width, logical_height) = ctx.gfx.drawable_size();
        let mut stations = StationHandler::new(
            ShapeBuilder::new(
                ctx,
                ShapePalette::new(Color::WHITE, Color::from_rgb(5, 5, 2)),
            ),
            ShapeBuilder::new(ctx, ShapePalette::fill(Color::from_rgb(5, 5, 2))),
        );
        stations.add_station(StationShape::Circle, Vec2::new(100.0, 100.0));
        stations.add_station(StationShape::Circle, Vec2::new(200.0, 200.0));
        stations.add_station(StationShape::Circle, Vec2::new(300.0, 100.0));
        stations.add_station(StationShape::Circle, Vec2::new(400.0, 200.0));
        stations.add_station(StationShape::Circle, Vec2::new(500.0, 100.0));

        let mut routes = RouteHandler::new();
        routes.add_route(
            vec![
                Stop::new(0, StopSide::Right),
                Stop::new(1, StopSide::Left),
                Stop::new(2, StopSide::Left),
                Stop::new(3, StopSide::Right),
            ],
            false,
        );
        routes.add_route(
            vec![
                Stop::new(4, StopSide::Right),
                Stop::new(3, StopSide::Left),
                Stop::new(2, StopSide::Left),
                Stop::new(1, StopSide::Right),
            ],
            false,
        );

        let mut metros = VehicleHandler::new(
            7,
            ShapeBuilder::new(
                ctx,
                ShapePalette::new(
                    Color::from_rgb(125, 125, 125),
                    Color::from_rgb(156, 156, 156),
                ),
            ),
        );
        metros.add_vehicle(Box::new(Metro::new(&ctx, 0)));
        metros.add_vehicle(Box::new(Metro::new(&ctx, 1)));

        MetroWorld {
            rng: StdRng::seed_from_u64(seed),
            time: 0.0,
            stations,
            routes,
            vehicles: metros,
            logical_width,
            logical_height,
            screen_transform_rect: Rect::new(0.0, 0.0, logical_width, logical_height),
        }
    }

    fn maintain_screen_aspect_ratio(&mut self, width: f32, height: f32) {
        let scale_x = width / self.logical_width;
        let scale_y = height / self.logical_height;
        let scale = scale_x.min(scale_y);

        let scaled_width = width / scale;
        let scaled_height = height / scale;

        self.screen_transform_rect = Rect::new(
            (self.logical_width - scaled_width) / 2.0,
            (self.logical_height - scaled_height) / 2.0,
            scaled_width,
            scaled_height,
        );
    }
}

impl EventHandler<GameError> for MetroWorld {
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.maintain_screen_aspect_ratio(width, height);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ctx.time.delta().as_secs_f32();

        self.stations.update(&mut self.rng, delta);
        self.routes.update(&ctx, &self.stations, delta);
        self.vehicles.update(delta, &self.routes, &mut self.stations);

        self.time += delta;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(self.screen_transform_rect);
        canvas.set_sampler(FilterMode::Linear);

        canvas.draw(
            &Quad,
            DrawParam::default()
                .color(Colors::background())
                .scale([self.logical_width, self.logical_height]),
        );

        for route in self.routes.iter_mut() {
            route.draw(&ctx, &mut canvas);
            for metro_id in self.vehicles.metros_on_route(route.id()) {
                self.vehicles
                    .get(metro_id)
                    .draw(&mut canvas, &self.vehicles.shapes(), route.color());
            }
        }
        self.stations.draw(&mut canvas, &self.vehicles);

        canvas.finish(ctx)
    }
}
