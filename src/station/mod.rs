use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam},
};
use types::StationKind;

use crate::{passenger::Passenger, shape::ShapeBuilder};

pub mod handler;
pub mod types;

pub struct Station {
    id: u32,
    kind: StationKind,
    size: f32,
    position: Vec2,
    passengers: Vec<Passenger>,
}

impl Station {
    pub fn new(id: u32, kind: StationKind, size: f32, position: Vec2) -> Self {
        Station {
            id,
            kind,
            size,
            position,
            passengers: vec![],
        }
    }

    pub fn kind(&self) -> StationKind {
        self.kind
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn size(&self) -> f32 {
        self.size * 15.0
    }

    pub fn draw(&self, canvas: &mut Canvas, shapes: &ShapeBuilder) {
        shapes
            .get_mesh(self.kind)
            .draw(canvas, DrawParam::default().scale([self.size, self.size]).dest(self.position));
        for passenger in self.passengers.iter() {
            shapes
                .get_mesh(passenger.kind())
                .draw(canvas, DrawParam::default().dest(self.position));
        }
    }
}
