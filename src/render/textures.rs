use crate::types::{Direction, VehicleColor};
use sdl2::{image::LoadTexture, render::Texture};
use std::collections::HashMap;

type TextureKey = (VehicleColor, Direction);

pub struct TextureCache<'a> {
    textures: HashMap<TextureKey, Texture<'a>>,
}

impl<'a> TextureCache<'a> {
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Self {
        let mut textures = HashMap::new();

        for color in [
            VehicleColor::Blue,
            VehicleColor::Green,
            VehicleColor::Pink,
            VehicleColor::Yellow,
        ] {
            for direction in [
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ] {
                let direction_str = match direction {
                    Direction::North => "north",
                    Direction::South => "south",
                    Direction::East => "east",
                    Direction::West => "west",
                };

                let color_str = match color {
                    VehicleColor::Blue => "blue",
                    VehicleColor::Green => "green",
                    VehicleColor::Pink => "pink",
                    VehicleColor::Yellow => "yellow",
                };

                let path = format!("assets/vehicles/{}/car_{}.png", direction_str, color_str);

                let texture = texture_creator
                    .load_texture(&path)
                    .expect("Failed to load texture");
                textures.insert((color, direction), texture);
            }
        }
        Self { textures }
    }

    pub fn get(&self, color: VehicleColor, direction: Direction) -> &Texture<'a> {
        &self.textures[&(color, direction)]
    }
}
