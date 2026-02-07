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
    pub intersection_entry_time: Option<f32>,
    pub max_speed_reached: f32,
    pub min_speed_reached: f32,
    pub had_close_call: bool,
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
            collision: Collision::new(x, y),
            intersection_entry_time: None,
            max_speed_reached: random_speed,
            min_speed_reached: random_speed,
            had_close_call: false,
        }
    }

    pub fn distance_to_intersection(&self) -> f32 {
        // Intersection center is at (400, 400)
        let intersection_center_x = 400.0;
        let intersection_center_y = 400.0;

        // Calculate Euclidean distance to intersection center
        ((self.x - intersection_center_x).powi(2) + (self.y - intersection_center_y).powi(2)).sqrt()
    }

    fn distance_to_stop_line(&self) -> f32 {
        match self.direction {
            Direction::South => 265.0 - self.y,
            Direction::North => self.y - 535.0,
            Direction::East => 265.0 - self.x,
            Direction::West => self.x - 535.0,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        // Update collision position
        self.collision.x = self.x;
        self.collision.y = self.y;

        // Calculate target speed based on conditions
        let mut target_speed = 1.5; // Default cruising speed
        let distance_to_intersection = self.distance_to_intersection();
        let distance_to_stop_line = self.distance_to_stop_line();

        // Slow down when approaching intersection
        if self.state == VehicleState::Approaching {
            let braking_distance = 140.0;
            if distance_to_stop_line < braking_distance {
                let braking_ratio = (distance_to_stop_line / braking_distance).max(0.2);
                target_speed = 1.5 * braking_ratio;
            } else if distance_to_intersection < 150.0 {
                let braking_ratio = (distance_to_intersection / 150.0).max(0.3);
                target_speed = 1.5 * braking_ratio;
            }
        }

        // Check for intersection mutual exclusion (only if at boundary)
        let in_intersection = Collision::is_vehicle_in_intersection(self);

        if self.state == VehicleState::Approaching
            && distance_to_stop_line <= 0.0
            && !in_intersection
            && Collision::should_wait_for_intersection(self, vehicles)
        {
            self.state = VehicleState::Waiting;
            target_speed = 0.0;
        }

        // Check for vehicles ahead and adjust target speed
        if let Some(vehicle_ahead) = Collision::check_vehicle_ahead(self, vehicles) {
            let distance =
                ((self.x - vehicle_ahead.x).powi(2) + (self.y - vehicle_ahead.y).powi(2)).sqrt();

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
            // Check if path is clear and intersection is available
            let path_clear = Collision::check_vehicle_ahead(self, vehicles).is_none();
            let intersection_available = !Collision::should_wait_for_intersection(self, vehicles);

            if path_clear && intersection_available {
                self.state = VehicleState::Approaching;
            } else {
                return;
            }
        }

        if Collision::is_vehicle_in_intersection(self) {
            if self.intersection_entry_time.is_none() {
                // Vehicle just entered intersection - start timing
                self.intersection_entry_time = Some(0.0);
            }
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
                            if self.x >= 400.5 {
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

        // Track speed statistics
        self.max_speed_reached = self.max_speed_reached.max(self.speed);
        self.min_speed_reached = self.min_speed_reached.min(self.speed);

        // Increment intersection time if vehicle is in intersection
        if let Some(ref mut entry_time) = self.intersection_entry_time {
            *entry_time += 1.0; // Increment by 1 frame
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
