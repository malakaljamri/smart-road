use crate::render::Sdl2Manager;
use crate::render::TextureCache;
use crate::traffic::Lane;
use crate::traffic::collision::Collision;
use crate::types::{Direction, VehicleColor, VehicleState};
use sdl2::rect::Rect;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub lane: Lane,
    pub direction: Direction,
    pub state: VehicleState,
    pub color: VehicleColor,
    pub speed: f32,
    pub collision: Collision,
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

        let mut rng = rand::thread_rng();
        let random_speed = rng.gen_range(0.8..1.2);
        
        Vehicle {
            id,
            x,
            y,
            speed: random_speed,
            color: random_color,
            direction: start_dir,
            lane,
            state: VehicleState::Approaching,
            collision: Collision::new(id as i32, x, y),
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        // Update collision position
        self.collision.x = self.x;
        self.collision.y = self.y;

        // Calculate target speed based on conditions
        let mut target_speed = 1.5; // Default cruising speed

        // Check for vehicles ahead and adjust target speed
        if let Some(vehicle_ahead) = Collision::check_vehicle_ahead(self, vehicles) {
            let distance = ((self.x - vehicle_ahead.x).powi(2) + (self.y - vehicle_ahead.y).powi(2)).sqrt();

            if distance < self.collision.safe_distance {
                self.state = VehicleState::Waiting;
                target_speed = 0.0;
            } else if distance < self.collision.safe_distance + 30.0 {
                // Slow down proportionally based on distance
                let ratio = (distance - self.collision.safe_distance) / 30.0;
                target_speed = 0.8 * ratio + 0.4; // Range: 0.4 to 1.2
            }
        }

        // Smoothly interpolate towards target speed
        if target_speed > self.speed {
            // Accelerate
            self.speed = (self.speed + 0.15).min(target_speed).min(2.0);
        } else if target_speed < self.speed {
            // Decelerate
            self.speed = (self.speed - 0.2).max(target_speed).max(0.3);
        }

        // If waiting, don't move
        if self.state == VehicleState::Waiting {
            // Check if path is clear now
            if Collision::check_vehicle_ahead(self, vehicles).is_none() {
                self.state = VehicleState::Approaching;
            } else {
                return;
            }
        }

        let is_in_intersection_bounds = [
            self.direction == Direction::South && self.y >= 295.0 && self.y <= 505.0,
            self.direction == Direction::North && self.y <= 505.0 && self.y >= 295.0,
            self.direction == Direction::East && self.x >= 295.0 && self.x <= 505.0,
            self.direction == Direction::West && self.x <= 505.0 && self.x >= 295.0,
        ];

        // Had to improve this by separating the condition for each direction
        if is_in_intersection_bounds.iter().any(|&x| x) {
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
                        }
                        Direction::West => {
                            if self.y >= 300.0 {
                                self.direction = Direction::West
                            }
                        }
                        _ => {}
                    }
                }
            }
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
            }
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
                        }
                        Direction::South => {
                            if self.x >= 300.0 {
                                self.direction = Direction::South
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn render(
        vehicles: &Vec<Self>,
        texture_cache: &TextureCache<'_>,
        sdl2_manager: &mut Sdl2Manager,
    ) {
        for vehicle in vehicles {
            let vehicle_texture = texture_cache.get(vehicle.color, vehicle.direction);

            sdl2_manager
                .canvas
                .copy(
                    &vehicle_texture,
                    None,
                    Some(Rect::new(vehicle.x as i32, vehicle.y as i32, 30, 30)),
                )
                .unwrap();
        }
    }
}
