use crate::{render::Vehicle, traffic::Lane, types::Direction, simulation::Simulation};
use rand::Rng;
use sdl2::keyboard::Keycode;

pub struct InputHandler {
    pub quit: bool,
    pub spawn_north: bool,
    pub spawn_south: bool,
    pub spawn_east: bool,
    pub spawn_west: bool,
    pub spawn_random: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            quit: false,
            spawn_north: false,
            spawn_south: false,
            spawn_east: false,
            spawn_west: false,
            spawn_random: false,
        }
    }

    pub fn handle_keydown(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Escape => self.quit = true,
            Keycode::Up => self.spawn_south = true,
            Keycode::Down => self.spawn_north = true,
            Keycode::Right => self.spawn_west = true,
            Keycode::Left => self.spawn_east = true,
            Keycode::R => {
                self.spawn_random = true;
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.spawn_north = false;
        self.spawn_south = false;
        self.spawn_east = false;
        self.spawn_west = false;
        self.spawn_random = false;
    }

    pub fn spawn_cars(&mut self, vehicles: &mut Vec<Vehicle>, simulation: &mut Simulation) {
        let mut rng = rand::thread_rng();

        // function for random
        if self.spawn_random {
            match rng.gen_range(0..4) {
                0 => self.spawn_north = true,
                1 => self.spawn_south = true,
                2 => self.spawn_east = true,
                _ => self.spawn_west = true,
            }
        }

        // Helper function to check if spawn position is safe for specific lane
        let is_lane_spawn_safe = |spawn_x: f32, spawn_y: f32, lane: &Lane, vehicles: &[Vehicle]| -> bool {
            let safe_distance = 40.0; // Use same safe_distance as collision system
            
            vehicles.iter().all(|v| {
                // Skip vehicles not in the same lane
                if v.lane.from != lane.from || v.lane.to != lane.to {
                    return true;
                }
                
                let distance = ((v.x - spawn_x).powi(2) + (v.y - spawn_y).powi(2)).sqrt();
                
                // Check if vehicle is too close to spawn position
                if distance < safe_distance {
                    return false;
                }
                
                // For vehicles further away, check if they're in the spawn path
                match lane.from {
                    Direction::South => {
                        // Vehicles spawning from south go north
                        // Check if existing vehicle is north of spawn point and in same lane path
                        !(v.y < spawn_y && (v.x - spawn_x).abs() < 20.0 && distance < safe_distance * 3.0)
                    }
                    Direction::North => {
                        // Vehicles spawning from north go south
                        !(v.y > spawn_y && (v.x - spawn_x).abs() < 20.0 && distance < safe_distance * 3.0)
                    }
                    Direction::East => {
                        // Vehicles spawning from east go west
                        !(v.x < spawn_x && (v.y - spawn_y).abs() < 20.0 && distance < safe_distance * 3.0)
                    }
                    Direction::West => {
                        // Vehicles spawning from west go east
                        !(v.x > spawn_x && (v.y - spawn_y).abs() < 20.0 && distance < safe_distance * 3.0)
                    }
                }
            })
        };

        // south to north
        if self.spawn_south {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 472.5),
                1 => (Direction::West, 402.5),
                _ => (Direction::North, 437.5),
            };

            let lane = Lane::set(Direction::South, random_dir);
            if is_lane_spawn_safe(x, 800.0, &lane, vehicles) {
                let vehicle = Vehicle::new(simulation.get_next_vehicle_id(), x, 800.0, lane);
                println!("vehicle: {:?}", vehicle);
                vehicles.push(vehicle);
            } else {
                println!("Lane South->{:?} congested, delaying spawn", random_dir);
            }
        }

        // north to south
        if self.spawn_north {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 365.0),
                1 => (Direction::West, 295.0),
                _ => (Direction::South, 330.0),
            };

            let lane = Lane::set(Direction::North, random_dir);
            if is_lane_spawn_safe(x, 0.0, &lane, vehicles) {
                let vehicle = Vehicle::new(simulation.get_next_vehicle_id(), x, 0.0, lane);
                println!("vehicle: {:?}", vehicle);
                vehicles.push(vehicle);
            } else {
                println!("Lane North->{:?} congested, delaying spawn", random_dir);
            }
        }

        // east to west
        if self.spawn_east {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 295.0),
                1 => (Direction::South, 365.0),
                _ => (Direction::West, 330.0),
            };

            let lane = Lane::set(Direction::East, random_dir);
            if is_lane_spawn_safe(800.0, y, &lane, vehicles) {
                let vehicle = Vehicle::new(simulation.get_next_vehicle_id(), 800.0, y, lane);
                println!("vehicle: {:?}", vehicle);
                vehicles.push(vehicle);
            } else {
                println!("Lane East->{:?} congested, delaying spawn", random_dir);
            }
        }

        // west to east
        if self.spawn_west {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 402.5),
                1 => (Direction::South, 472.5),
                _ => (Direction::East, 437.5),
            };

            let lane = Lane::set(Direction::West, random_dir);
            if is_lane_spawn_safe(0.0, y, &lane, vehicles) {
                let vehicle = Vehicle::new(simulation.get_next_vehicle_id(), 0.0, y, lane);
                println!("vehicle: {:?}", vehicle);
                vehicles.push(vehicle);
            } else {
                println!("Lane West->{:?} congested, delaying spawn", random_dir);
            }
        }
    }
}

// move spawn code to vehicle
