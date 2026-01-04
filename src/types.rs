#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub state: VehicleState,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VehicleState {
    Approaching,
    Waiting,
    Crossing,
    Exiting,
}
