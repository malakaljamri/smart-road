#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum VehicleColor {
    Blue,
    Green,
    Pink,
    Yellow,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum VehicleState {
    Approaching,
    Waiting,
    Crossing,
    Exiting,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Lane {
    pub from: Direction,
    pub to: Direction,
}

impl Lane {
    pub fn set(from: Direction, to: Direction) -> Self {
        Lane { from, to }
    }
}
