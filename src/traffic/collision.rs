use crate::render::Vehicle;

#[derive(Debug)]
pub struct Collision {
    pub vehicle_id: i32,
    pub x: f32,
    pub y: f32,
    pub safe_distance: f32,
}

impl Collision {
    pub fn new(vehicle_id: i32, x: f32, y: f32) -> Self {
        Self {
            vehicle_id,
            x,
            y,
            safe_distance: 35.0,
        }
    }
}
