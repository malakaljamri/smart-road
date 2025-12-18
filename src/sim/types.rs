#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Route {
    Right,
    Straight,
    Left,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LaneId {
    pub from: Direction,
    pub route: Route,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VehicleState {
    Approaching,
    Waiting,
    Crossing,
    Exited,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VelocityLevel {
    Slow,
    Medium,
    Fast,
}