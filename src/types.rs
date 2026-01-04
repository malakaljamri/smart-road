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

pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub state: VehicleState,
    pub color: VehicleColor,
    pub speed: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VehicleState {
    Approaching,
    Waiting,
    Crossing,
    Exiting,
}
