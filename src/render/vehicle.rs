use crate::types::{Direction, VehicleState};

pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub state: VehicleState,
}

impl Vehicle {
    pub fn new(id: usize, x: f32, y: f32, direction: Direction) -> Self {
        Vehicle {
            id,
            x,
            y,
            direction,
            state: VehicleState::Approaching,
        }
    }

    pub fn update(&mut self) {
        // update logic here
    }
}
