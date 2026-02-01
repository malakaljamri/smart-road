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
            safe_distance: 80.0, // Increased from 60.0 for safer following distance
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
        // Increased check distance for earlier detection
        let check_distance = vehicle.collision.safe_distance + 40.0; // Increased from 20.0
        
        vehicles.iter().find(|other| {
            other.id != vehicle.id && Collision::is_vehicle_in_path(vehicle, other, check_distance)
        })
    }

    pub fn is_vehicle_in_path(vehicle: &Vehicle, other: &Vehicle, check_distance: f32) -> bool {
        let distance = ((vehicle.x - other.x).powi(2) + (vehicle.y - other.y).powi(2)).sqrt();
        
        if distance > check_distance {
            return false;
        }

        // Increased lateral tolerance from 20.0 to 25.0 for more safety margin
        match vehicle.direction {
            Direction::North => other.y < vehicle.y && (other.x - vehicle.x).abs() < 25.0,
            Direction::South => other.y > vehicle.y && (other.x - vehicle.x).abs() < 25.0,
            Direction::East => other.x > vehicle.x && (other.y - vehicle.y).abs() < 25.0,
            Direction::West => other.x < vehicle.x && (other.y - vehicle.y).abs() < 25.0,
        }
    }

    pub fn is_vehicle_in_intersection(vehicle: &Vehicle) -> bool {
        // Added safety buffer around intersection bounds
        // Original: 295-505, Now: 285-515 (10 unit buffer)
        vehicle.x >= 285.0 && vehicle.x <= 515.0 && vehicle.y >= 285.0 && vehicle.y <= 515.0
    }

    pub fn count_vehicles_in_intersection(vehicles: &[Vehicle], exclude_id: usize) -> usize {
        vehicles.iter().filter(|v| v.id != exclude_id && Self::is_vehicle_in_intersection(v)).count()
    }

    pub fn has_vehicle_in_intersection(vehicles: &[Vehicle], exclude_id: usize) -> bool {
        vehicles.iter().any(|v| v.id != exclude_id && Self::is_vehicle_in_intersection(v))
    }

    pub fn should_wait_for_intersection(vehicle: &Vehicle, vehicles: &[Vehicle]) -> bool {
        // Reduced maximum vehicles in intersection from 4 to 3 for safer spacing
        let vehicles_in_intersection = Self::count_vehicles_in_intersection(vehicles, vehicle.id);
        vehicles_in_intersection >= 2 // Changed from 4
    }
}

use crate::types::Direction;