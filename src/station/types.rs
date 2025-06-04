#[derive(strum_macros::EnumIter, strum_macros::EnumCount, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StationShape {
    Circle = 0,
    Square = 1,
    Diamond = 2,
    Triangle = 3,
}

#[derive(strum_macros::EnumIter, strum_macros::EnumCount, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StationType {
    Normal = 0
}