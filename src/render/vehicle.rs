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
            speed: 0.5,
            color: random_color,
            direction: start_dir,
            lane,
            state: VehicleState::Approaching,
        }
    }

    pub fn update(&mut self) {
        // Test movement logic: move the vehicle in its direction
        // if self.y > 295.0 && self.y < 505.0 && self.x > 295.0 && self.x < 505.0 {
        //     self.state = VehicleState::Crossing;
        // }

        // Had to improve this by separating the condition for each direction
        if (self.direction == Direction::South && self.y >= 295.0 && self.y <= 505.0)
            || (self.direction == Direction::North && self.y <= 505.0 && self.y >= 295.0)
            || (self.direction == Direction::East && self.x >= 295.0 && self.x <= 505.0)
            || (self.direction == Direction::West && self.x <= 505.0 && self.x >= 295.0)
        {
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
            Direction::South => {
                self.y += self.speed;

                if self.y > 505.0 {
                    self.state = VehicleState::Exiting;
                }

                if self.state == VehicleState::Crossing {
                    match self.lane.to {
                        Direction::East => {
                            if self.y >= 395.0 {
                                self.direction = Direction::East
                            }
                        },
                        Direction::West => {
                            if self.y >= 300.0 {
                                self.direction = Direction::West
                            }
                        },
                        _ => {}
                    }
                }
            },
            Direction::West => {
                self.x -= self.speed;

                if self.x < 295.0 {
                    self.state = VehicleState::Exiting;
                }

                if self.state == VehicleState::Crossing {
                    match self.lane.to {
                        Direction::North => {
                            if self.x <= 472.5 {
                                self.direction = Direction::North;
                            }
                        }
                        Direction::South => {
                            if self.x <= 367.5 {
                                self.direction = Direction::South;
                            }
                        }
                        _ => {}
                    }
                }
            },
            Direction::East => {
                self.x += self.speed;

                if self.x > 505.0 {
                    self.state = VehicleState::Exiting;
                }

                if self.state == VehicleState::Crossing {
                    match self.lane.to {
                        Direction::North => {
                            if self.x >= 395.0 {
                                self.direction = Direction::North
                            }
                        },
                        Direction::South => {
                            if self.x >= 300.0 {
                                self.direction = Direction::South
                            }
                        },
                        _ => {}
                    }
                }
            },
        }
    }
}
