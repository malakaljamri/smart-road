use crate::intersection::Lane;
use crate::types::{Direction, VehicleColor, VehicleState};
use rand::Rng;

#[derive(Debug)]
pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub lane: Lane,
    pub direction: Direction,
    pub state: VehicleState,
    pub color: VehicleColor,
    pub speed: f32,
}

impl Vehicle {
    pub fn new(id: usize, x: f32, y: f32, lane: Lane) -> Self {
        let mut rng = rand::thread_rng();
        let random_color = match rng.gen_range(0..4) {
            0 => VehicleColor::Blue,
            1 => VehicleColor::Green,
            2 => VehicleColor::Pink,
            _ => VehicleColor::Yellow,
        };

        let start_dir = match lane.from {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        };

        Vehicle {
            id,
            x,
            y,
            //TODO: Adjust speed based on project requirements
            speed: 1.0,
            color: random_color,
            direction: start_dir,
            lane,
            state: VehicleState::Approaching,
        }
    }

    pub fn update(&mut self) {
        // Test movement logic: move the vehicle in its direction
        if self.y > 295.0 && self.y < 505.0 && self.x > 295.0 && self.x < 505.0 {
            self.state = VehicleState::Crossing;
        }

        //TODO: Implement proper logic
        match self.direction {
            Direction::North => {
                self.y -= self.speed;

                if self.y < 295.0 {
                    self.state = VehicleState::Exiting;
                }

                if self.state == VehicleState::Crossing {
                    match self.lane.to {
                        Direction::East => {
                            if self.y <= 472.5 {
                                self.direction = Direction::East;
                            }
                        }
                        Direction::West => {
                            if self.y <= 367.5 {
                                self.direction = Direction::West;
                            }
                        }
                        _ => {}
                    }
                }
            }
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
