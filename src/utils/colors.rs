use ggez::graphics::Color;

pub struct Colors {}

impl Colors {
    pub fn background() -> Color {
        Color::from_rgb(237, 237, 235)
    }

    pub fn default_palette() -> Vec<Color> {
        vec![
            Color::from_rgb(235, 64, 52),
            Color::from_rgb(235, 167, 59),
            Color::from_rgb(232, 222, 30),
            Color::from_rgb(34, 240, 123),
            Color::from_rgb(18, 159, 219),
            Color::from_rgb(30, 19, 235),
            Color::from_rgb(184, 12, 232),
        ]
    }
}