use crate::station::types::{StationShape, StationType};

pub struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    size: f32,
    cells: Vec<Vec<Option<(StationShape, StationType)>>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            size: 15.0,
            cells: vec![vec![None; height]; width],
        }
    }

    pub fn can_fill(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.cells[x][y].is_none()
    }

    pub fn fill(&mut self, x: usize, y: usize, shape: StationShape, station_type: StationType) {
        self.cells[x][y] = Some((shape, station_type));
    }
}