use crate::types::{Direction, Vehicle, VehicleState};

impl Vehicle {
    pub fn new(id: usize, x: f32, y: f32, direction: Direction) -> Self {
        Vehicle {
            id,
            x,
            y,
            speed: 1.0,
            direction,
            state: VehicleState::Approaching,
        }
    }

    pub fn update(&mut self) {
        // Test movement logic: move the vehicle in its direction
        match self.direction {
            Direction::North => self.y -= self.speed,
            Direction::South => self.y += self.speed,
            Direction::East => self.x += self.speed,
            Direction::West => self.x -= self.speed,
        }
    }
}
