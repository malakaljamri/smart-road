#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VehicleColor {
    Blue,
    Green,
    Pink,
    Yellow,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VehicleState {
    Approaching,
    Waiting,
    Crossing,
    Exiting,
}
