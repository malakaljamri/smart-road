use crate::types::{Direction, Vehicle, VehicleState};

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
        // Test movement logic: move the vehicle in its direction
        match self.direction {
            Direction::North => self.y -= 2.0,
            Direction::South => self.y += 2.0,
            Direction::East => self.x += 2.0,
            Direction::West => self.x -= 2.0,
        }
    }
}
