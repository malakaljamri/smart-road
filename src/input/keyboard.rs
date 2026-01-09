// use crate::types::Direction;
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
