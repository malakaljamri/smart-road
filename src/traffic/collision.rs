use crate::render::Vehicle;

#[derive(Debug, Clone)]
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
            safe_distance: 40.0,
        }
    }

    pub fn check_collision(vehicle1: &Vehicle, vehicle2: &Vehicle) -> bool {
        if vehicle1.id == vehicle2.id {
            return false;
        }

        let distance = ((vehicle1.x - vehicle2.x).powi(2) + (vehicle1.y - vehicle2.y).powi(2)).sqrt();
        distance < vehicle1.collision.safe_distance
    }

    pub fn check_vehicle_ahead<'a>(vehicle: &'a Vehicle, vehicles: &'a [Vehicle]) -> Option<&'a Vehicle> {
        let check_distance = vehicle.collision.safe_distance + 20.0;
        
        vehicles.iter().find(|other| {
            other.id != vehicle.id && Collision::is_vehicle_in_path(vehicle, other, check_distance)
        })
    }

    pub fn is_vehicle_in_path(vehicle: &Vehicle, other: &Vehicle, check_distance: f32) -> bool {
        let distance = ((vehicle.x - other.x).powi(2) + (vehicle.y - other.y).powi(2)).sqrt();
        
        if distance > check_distance {
            return false;
        }

        match vehicle.direction {
            Direction::North => other.y < vehicle.y && (other.x - vehicle.x).abs() < 20.0,
            Direction::South => other.y > vehicle.y && (other.x - vehicle.x).abs() < 20.0,
            Direction::East => other.x > vehicle.x && (other.y - vehicle.y).abs() < 20.0,
            Direction::West => other.x < vehicle.x && (other.y - vehicle.y).abs() < 20.0,
        }
    }

    pub fn is_vehicle_in_intersection(vehicle: &Vehicle) -> bool {
        // Check if vehicle is within intersection bounds (295-505 on both axes)
        vehicle.x >= 295.0 && vehicle.x <= 505.0 && vehicle.y >= 295.0 && vehicle.y <= 505.0
    }

    pub fn has_vehicle_in_intersection(vehicles: &[Vehicle], exclude_id: usize) -> bool {
        vehicles.iter().any(|v| v.id != exclude_id && Self::is_vehicle_in_intersection(v))
    }

    pub fn should_wait_for_intersection(vehicle: &Vehicle, vehicles: &[Vehicle]) -> bool {
        // Right-turning vehicles can proceed without waiting (real-world behavior)
        let is_right_turn = match vehicle.lane.from {
            Direction::North => vehicle.lane.to == Direction::East,
            Direction::South => vehicle.lane.to == Direction::West,
            Direction::East => vehicle.lane.to == Direction::North,
            Direction::West => vehicle.lane.to == Direction::South,
        };

        if is_right_turn {
            return false; // Right turns don't wait for intersection
        }

        // Straight and left-turning vehicles must wait if intersection is occupied
        Self::has_vehicle_in_intersection(vehicles, vehicle.id)
    }
}

use crate::types::Direction;
