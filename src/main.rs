use ggez::{
    ContextBuilder,
    conf::{NumSamples, WindowMode, WindowSetup},
    event::run,
};
use world::MetroWorld;

mod passenger;
mod route;
mod shape;
mod station;
mod vehicle;
mod utils;
mod world;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("rusty-metro", "waltsai")
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(true)
                .min_dimensions(640.0, 360.0)
                .max_dimensions(1920.0, 1080.0),
        )
        .window_setup(
            WindowSetup::default()
                .title("Rusty Metro")
                .icon("/icon.png")
                .samples(NumSamples::Four),
        )
        .build()
        .expect("Failed to create ggez context!");

    let game = MetroWorld::new(&mut ctx, 41);
    run(ctx, event_loop, game);
}
