use std::f32::consts::PI;

use ggez::{
    Context,
    glam::Vec2,
    graphics::{Canvas, DrawMode, DrawParam, Mesh, MeshBuilder, Rect},
};
use palette::ShapePalette;
use strum::IntoEnumIterator;

use crate::station::types::StationKind;

pub mod palette;

pub struct ShapeBuilder {
    shapes: Vec<Shape>,
}

impl ShapeBuilder {
    pub fn new(ctx: &mut Context, shape_color: ShapePalette) -> Self {
        ShapeBuilder {
            shapes: StationKind::iter()
                .map(|t| ShapeBuilder::create_mesh(ctx, &shape_color, t))
                .collect(),
        }
    }

    pub fn get_mesh(&self, shape_type: StationKind) -> Shape {
        self.shapes[shape_type as usize].clone()
    }

    pub fn create_mesh(
        ctx: &Context,
        shape_color: &ShapePalette,
        shape_type: StationKind,
    ) -> Shape {
        let mb = &mut MeshBuilder::new();
        match shape_type {
            StationKind::Circle => {
                mb.circle(
                    DrawMode::fill(),
                    [0.0, 0.0],
                    15.0,
                    0.1,
                    shape_color.filled(),
                )
                .expect("Error creating mesh for station.");
                mb.circle(
                    DrawMode::stroke(5.0),
                    [0.0, 0.0],
                    15.0,
                    0.1,
                    shape_color.outline(),
                )
                .expect("Error creating mesh for station.");
                Shape::Circle(Mesh::from_data(ctx, mb.build()))
            }
            StationKind::Square => {
                mb.rectangle(
                    DrawMode::fill(),
                    Rect::new(-12.0, -12.0, 24.0, 24.0),
                    shape_color.filled(),
                )
                .expect("Error creating mesh for station.");
                mb.rectangle(
                    DrawMode::stroke(5.0),
                    Rect::new(-12.0, -12.0, 24.0, 24.0),
                    shape_color.outline(),
                )
                .expect("Error creating mesh for station.");
                Shape::Square(Mesh::from_data(ctx, mb.build()))
            }
            StationKind::Diamond => {
                mb.rectangle(
                    DrawMode::fill(),
                    Rect::new(-12.0, -12.0, 24.0, 24.0),
                    shape_color.filled(),
                )
                .expect("Error creating mesh for station.");
                mb.rectangle(
                    DrawMode::stroke(5.0),
                    Rect::new(-12.0, -12.0, 24.0, 24.0),
                    shape_color.outline(),
                )
                .expect("Error creating mesh for station.");
                Shape::Diamond(Mesh::from_data(ctx, mb.build()))
            }
            StationKind::Triangle => {
                let vertices = [PI / 2.0, 7.0 * PI / 6.0, 11.0 * PI / 6.0];
                mb.triangles(
                    &vertices.map(|a| Vec2::from_angle(a) * 24.0),
                    shape_color.outline(),
                )
                .expect("Error creating mesh for station.");
                mb.triangles(
                    &vertices.map(|a| Vec2::from_angle(a) * 19.0),
                    shape_color.filled(),
                )
                .expect("Error creating mesh for station.");
                Shape::Triangle(Mesh::from_data(ctx, mb.build()))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Shape {
    Circle(Mesh),
    Square(Mesh),
    Diamond(Mesh),
    Triangle(Mesh),
}

impl Shape {
    pub fn draw(&self, canvas: &mut Canvas, draw_param: DrawParam) {
        match self {
            Self::Circle(mesh)
            | Self::Square(mesh)
            | Self::Diamond(mesh)
            | Self::Triangle(mesh) => {
                canvas.draw(
                    mesh,
                    draw_param.rotation(if let Shape::Diamond(_) = self {
                        PI / 4.0
                    } else {
                        0.0
                    }),
                );
            }
        }
    }
}
