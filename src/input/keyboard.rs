use crate::{intersection::Lane, render::Vehicle, types::Direction};
use rand::Rng;
use sdl2::keyboard::Keycode;

pub struct InputHandler {
    pub quit: bool,
    pub spawn_north: bool,
    pub spawn_south: bool,
    pub spawn_east: bool,
    pub spawn_west: bool,
    pub toggle_random: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            quit: false,
            spawn_north: false,
            spawn_south: false,
            spawn_east: false,
            spawn_west: false,
            toggle_random: false,
        }
    }

    pub fn handle_keydown(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Escape => self.quit = true,
            Keycode::Up => self.spawn_north = true,
            Keycode::Down => self.spawn_south = true,
            Keycode::Right => self.spawn_east = true,
            Keycode::Left => self.spawn_west = true,
            Keycode::R => self.toggle_random = true,
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.spawn_north = false;
        self.spawn_south = false;
        self.spawn_east = false;
        self.spawn_west = false;
        self.toggle_random = false;
    }

    pub fn spawn_cars(&mut self, vehicles: &mut Vec<Vehicle>) {
        let mut rng = rand::thread_rng();

        // south to north
        if self.spawn_south {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 472.5),
                1 => (Direction::West, 402.5),
                _ => (Direction::North, 437.5),
            };

            let lane = Lane::set(Direction::South, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), x, 800.0, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // north to south
        if self.spawn_north {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 365.0),
                1 => (Direction::West, 295.0),
                _ => (Direction::South, 330.0),
            };

            let lane = Lane::set(Direction::North, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), x, 0.0, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // east to west
        if self.spawn_east {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 295.0),
                1 => (Direction::South, 365.0),
                _ => (Direction::West, 330.0),
            };

            let lane = Lane::set(Direction::East, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), 800.0, y, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // west to east
        if self.spawn_west {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 402.5),
                1 => (Direction::South, 472.5),
                _ => (Direction::East, 437.5),
            };

            let lane = Lane::set(Direction::West, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), 0.0, y, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }
    }

    //? Might be useful later
    // pub fn get_direction(&self) -> Option<Direction> {
    //     if self.spawn_north {
    //         Some(Direction::North)
    //     } else if self.spawn_south {
    //         Some(Direction::South)
    //     } else if self.spawn_east {
    //         Some(Direction::East)
    //     } else if self.spawn_west {
    //         Some(Direction::West)
    //     } else {
    //         None
    //     }
    // }
}
