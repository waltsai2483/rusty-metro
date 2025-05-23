use ggez::{
    Context, GameError, GameResult,
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, FilterMode, Quad, Rect},
};
use rand::rngs::ThreadRng;

use crate::{
    route::{
        handler::RouteHandler, stop::{Stop, StopSide}, Route
    },
    shape::palette::ShapePalette,
    station::{handler::StationHandler, types::StationKind},
    utils::colors::Colors, vehicle::{handler::VehicleHandler, metro::Metro},
};

pub struct MetroWorld {
    rng: ThreadRng,
    time: f32,

    logical_width: f32,
    logical_height: f32,
    screen_transform_rect: Rect,

    stations: StationHandler,
    routes: RouteHandler,
    metros: VehicleHandler
}

impl MetroWorld {
    pub fn new(ctx: &mut Context) -> Self {
        let (logical_width, logical_height) = ctx.gfx.drawable_size();
        let mut stations = StationHandler::new(
            ctx,
            ShapePalette::new(Color::WHITE, Color::from_rgb(5, 5, 2)),
        );
        stations.add_station(StationKind::Circle, Vec2::new(100.0, 100.0));
        stations.add_station(StationKind::Circle, Vec2::new(200.0, 200.0));
        stations.add_station(StationKind::Circle, Vec2::new(300.0, 100.0));
        stations.add_station(StationKind::Circle, Vec2::new(400.0, 200.0));
        stations.add_station(StationKind::Circle, Vec2::new(500.0, 100.0));

        let mut metros = VehicleHandler::new();
        metros.add_vehicle(Box::new(Metro::new(&ctx, 0, Color::RED)));

        MetroWorld {
            rng: rand::rng(),
            time: 0.0,
            stations,
            routes: RouteHandler::new(vec![Route::new(
                vec![
                    Stop::new(0, StopSide::Right),
                    Stop::new(1, StopSide::Right),
                    Stop::new(2, StopSide::Right),
                    Stop::new(3, StopSide::Right),
                ],
                false,
            )]),
            metros: metros,
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
        let mut delta = ctx.time.delta().as_secs_f32();

        self.routes.update(&ctx, &self.stations, delta);
        self.metros.update(delta, &self.routes, &self.stations);

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
        self.routes.draw(ctx, &mut canvas, &self.stations);
        self.stations.draw(&mut canvas);
        self.metros.draw(&mut canvas, &self.stations.shapes());

        canvas.finish(ctx)
    }
}
