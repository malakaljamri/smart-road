use crate::types::{Direction, Vehicle, VehicleColor, VehicleState};
use rand::Rng;

impl Vehicle {
    pub fn new(id: usize, x: f32, y: f32, direction: Direction) -> Self {
        let mut rng = rand::thread_rng();
        let random_color = match rng.gen_range(0..4) {
            0 => VehicleColor::Blue,
            1 => VehicleColor::Green,
            2 => VehicleColor::Pink,
            _ => VehicleColor::Yellow,
        };

        Vehicle {
            id,
            x,
            y,
            //TODO: Adjust speed based on project requirements
            speed: 1.0,
            color: random_color,
            direction,
            state: VehicleState::Approaching,
        }
    }

    pub fn update(&mut self) {
        // Test movement logic: move the vehicle in its direction
        //TODO: Implement proper logic
        match self.direction {
            Direction::North => self.y -= self.speed,
            Direction::South => self.y += self.speed,
            Direction::East => self.x += self.speed,
            Direction::West => self.x -= self.speed,
        }
    }

    pub fn get_texture_path(&self) -> String {
        let direction_str = match self.direction {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        };

        let color_str = match self.color {
            VehicleColor::Blue => "blue",
            VehicleColor::Green => "green",
            VehicleColor::Pink => "pink",
            VehicleColor::Yellow => "yellow",
        };

        format!("assets/vehicles/{}/car_{}.png", direction_str, color_str)
    }
}
