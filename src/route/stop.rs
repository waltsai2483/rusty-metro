#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StopSide {
    Left = -1,
    Right = 1
}

#[derive(Clone, Copy)]
pub struct Stop {
    index: usize,
    side: StopSide
}

impl Stop {
    pub fn new(index: usize, side: StopSide) -> Self {
        Stop { index, side }
    }

    pub fn index(&self) -> usize { self.index }

    pub fn side(&self) -> StopSide { self.side }

    pub fn side_factor(&self) -> f32 { self.side as i32 as f32 }
}